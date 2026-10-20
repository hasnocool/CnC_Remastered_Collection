#!/usr/bin/env python3
"""Turn an Agent Studio oracle request into intent answers.

The map editor exports two files next to the scenario:

    ai_oracle_request.json   machine readable observation + intent slots
    ai_oracle_prompt.txt     ready-to-send prompt (observation + vision grid)

This script sends the prompt to an OpenAI-compatible chat endpoint, validates
the JSON answer against the requested slots and writes
ai_oracle_response.json, which the editor and launcher load as the agent's
IntentCache. Omitted keys are allowed: the script's deterministic fallback
then applies.

Environment:
    CNC_LLM_BASE_URL   default http://localhost:11434/v1 (Ollama)
    CNC_LLM_MODEL      default llama3.1
    CNC_LLM_API_KEY    optional bearer token (hosted providers)
    CNC_LLM_TIMEOUT    seconds, default 60
    CNC_LLM_VISION     set to 1 to attach ai_vision.png (multimodal models)

Usage:
    python scripts/ai_oracle.py [--request PATH] [--response PATH] [--dry-run]

The simulation never calls this script. It runs ahead of time (or between
ticks in a hosting service) and the answers are recorded as part of the match,
so lockstep stays deterministic.
"""

import argparse
import base64
import json
import os
import re
import sys
import urllib.error
import urllib.request
from pathlib import Path

DEFAULT_REQUEST = "ai_oracle_request.json"
DEFAULT_RESPONSE = "ai_oracle_response.json"


def load_request(path: Path) -> dict:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except OSError as error:
        raise SystemExit(f"cannot read {path}: {error}") from error
    except json.JSONDecodeError as error:
        raise SystemExit(f"{path} is not valid JSON: {error}") from error


def build_prompt(request: dict, prompt_path: Path) -> str:
    if prompt_path.exists():
        return prompt_path.read_text(encoding="utf-8")
    lines = [
        "You are the strategic advisor for a Command & Conquer AI commander.",
        f"Agent: {request.get('agent', '?')} | Owner: {request.get('owner', '?')}",
        f"Objective: {request.get('objective', '')}",
        "",
        "Observation (JSON):",
        json.dumps(request.get("observation", {}), indent=2),
        "",
        "Choose a value for each requested intent:",
    ]
    for slot in request.get("intents", []):
        lines.append(
            f"- key {slot['key']!r}, kind {slot['kind']!r}, "
            f"fallback {slot['fallback']!r}"
        )
    lines.append("")
    lines.append(
        'Reply with JSON only: {"answers": [{"key": "...", "value": 0}]}.'
    )
    return "\n".join(lines)


def fallback_response(request: dict) -> dict:
    answers = []
    for slot in request.get("intents", []):
        fallback = slot.get("fallback", "")
        if slot.get("kind") == "int":
            try:
                value = int(fallback)
            except (TypeError, ValueError):
                value = 0
        else:
            value = fallback
        answers.append({"key": slot["key"], "value": value})
    return {"request_id": request.get("request_id"), "answers": answers, "note": "fallback"}


def image_content(prompt: str, image_path: Path):
    enabled = os.environ.get("CNC_LLM_VISION", "").strip().lower() in {"1", "true", "yes"}
    if not enabled or not image_path.exists():
        return prompt
    mime = "image/png" if image_path.suffix.lower() == ".png" else "image/jpeg"
    encoded = base64.b64encode(image_path.read_bytes()).decode("ascii")
    return [
        {"type": "text", "text": prompt},
        {
            "type": "image_url",
            "image_url": {"url": f"data:{mime};base64,{encoded}"},
        },
    ]


def call_model(prompt: str, image_path: Path) -> str:
    base_url = os.environ.get("CNC_LLM_BASE_URL", "http://localhost:11434/v1").rstrip("/")
    model = os.environ.get("CNC_LLM_MODEL", "llama3.1")
    timeout = float(os.environ.get("CNC_LLM_TIMEOUT", "60"))
    body = json.dumps(
        {
            "model": model,
            "temperature": 0.2,
            "messages": [
                {
                    "role": "system",
                    "content": "You output only compact JSON that matches the requested schema.",
                },
                {
                    "role": "user",
                    "content": image_content(prompt, image_path),
                },
            ],
        }
    ).encode("utf-8")
    headers = {"Content-Type": "application/json"}
    api_key = os.environ.get("CNC_LLM_API_KEY")
    if api_key:
        headers["Authorization"] = f"Bearer {api_key}"
    request = urllib.request.Request(
        f"{base_url}/chat/completions", data=body, headers=headers, method="POST"
    )
    try:
        with urllib.request.urlopen(request, timeout=timeout) as response:
            payload = json.loads(response.read().decode("utf-8"))
    except urllib.error.URLError as error:
        raise SystemExit(f"LLM request failed: {error}") from error
    try:
        return payload["choices"][0]["message"]["content"]
    except (KeyError, IndexError, TypeError) as error:
        raise SystemExit(f"unexpected LLM response: {payload}") from error


def extract_json(text: str) -> dict:
    text = text.strip()
    fenced = re.search(r"\{.*\}", text, re.S)
    if fenced:
        text = fenced.group(0)
    try:
        return json.loads(text)
    except json.JSONDecodeError as error:
        raise SystemExit(f"model did not return JSON: {error}\n{text[:400]}") from error


def validate(request: dict, response: dict) -> dict:
    slots = {slot["key"]: slot for slot in request.get("intents", [])}
    answers = []
    for answer in response.get("answers", []):
        key = answer.get("key")
        value = answer.get("value")
        if key not in slots:
            print(f"dropping unknown intent key {key!r}", file=sys.stderr)
            continue
        slot = slots[key]
        if slot.get("kind") == "int":
            if isinstance(value, bool) or not isinstance(value, int):
                print(f"dropping non-integer answer for {key!r}", file=sys.stderr)
                continue
        elif not isinstance(value, str):
            print(f"dropping non-text answer for {key!r}", file=sys.stderr)
            continue
        answers.append({"key": key, "value": value})
    return {
        "request_id": request.get("request_id"),
        "answers": answers,
        "note": response.get("note"),
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--request", default=DEFAULT_REQUEST)
    parser.add_argument("--response", default=DEFAULT_RESPONSE)
    parser.add_argument("--dry-run", action="store_true", help="write deterministic fallbacks")
    args = parser.parse_args()

    request_path = Path(args.request)
    response_path = Path(args.response)
    request = load_request(request_path)
    prompt = build_prompt(request, request_path.with_name("ai_oracle_prompt.txt"))
    image_path = Path(request.get("vision_image") or request_path.with_name("ai_vision.png"))

    if args.dry_run:
        response = fallback_response(request)
    else:
        response = validate(request, extract_json(call_model(prompt, image_path)))

    response_path.write_text(json.dumps(response, indent=2) + "\n", encoding="utf-8")
    answered = len(response.get("answers", []))
    print(f"wrote {response_path} ({answered} answer(s))")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
