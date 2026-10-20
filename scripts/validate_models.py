#!/usr/bin/env python3
"""
validate_models.py
Authentic validation script for Oklo Pool Mobile & Planetary Explorer v2
epistemic model records in data/models/manifest.json against data/models/schema.json.

This script executes both full JSON Schema Draft 2020-12 validation (if jsonschema
is available) and standalone native constraint validation (types, required fields,
regex patterns, enums, and additionalProperties: false).
"""

import json
import re
import sys
from pathlib import Path

def validate_models(schema_path: str, manifest_path: str) -> bool:
    print(f"=== Epistemic Model Manifest Validation ===")
    print(f"Schema:   {schema_path}")
    print(f"Manifest: {manifest_path}")

    # 1. Parse JSON files
    try:
        with open(schema_path, "r", encoding="utf-8") as f:
            schema = json.load(f)
        print("PASS: schema.json is valid JSON.")
    except Exception as e:
        print(f"FAIL: schema.json parse error: {e}")
        return False

    try:
        with open(manifest_path, "r", encoding="utf-8") as f:
            manifest = json.load(f)
        print("PASS: manifest.json is valid JSON.")
    except Exception as e:
        print(f"FAIL: manifest.json parse error: {e}")
        return False

    models = manifest.get("models", [])
    total_models = len(models)
    print(f"Total models registered: {total_models}")
    if total_models != 17:
        print(f"FAIL: Expected exactly 17 models, found {total_models}")
        return False

    # 2. Check if jsonschema library is available
    has_jsonschema = False
    try:
        import jsonschema
        has_jsonschema = True
        try:
            import importlib.metadata
            v = importlib.metadata.version("jsonschema")
        except Exception:
            v = getattr(jsonschema, "__version__", "unknown")
        print(f"jsonschema engine available (v{v}).")
    except ImportError:
        print("Notice: 'jsonschema' package not installed in current environment; running native strict schema validator.")

    schema_props = schema.get("properties", {})
    required_fields = set(schema.get("required", []))
    allowed_props = set(schema_props.keys())
    model_id_pattern = re.compile(schema_props.get("model_id", {}).get("pattern", ".*"))
    source_commit_pattern = re.compile(schema_props.get("source_commit", {}).get("pattern", ".*"))
    allowed_evidence_classes = set(schema_props.get("evidence_class", {}).get("enum", []))

    failures = 0
    empirically_calibrated_count = 0

    print("\n--- Model-by-Model Validation ---")
    for idx, model in enumerate(models):
        mid = model.get("model_id", f"<unknown_at_{idx}>")
        model_errors = []

        # JSON Schema Draft 2020-12 validation
        if has_jsonschema:
            try:
                jsonschema.validate(instance=model, schema=schema)
            except jsonschema.ValidationError as err:
                model_errors.append(f"jsonschema: {err.message}")

        # Native strict rules validation
        if not isinstance(model, dict):
            model_errors.append("Model record must be a JSON object")
        else:
            # Check required fields
            missing = required_fields - set(model.keys())
            if missing:
                model_errors.append(f"Missing required fields: {sorted(missing)}")

            # Check additionalProperties
            extra = set(model.keys()) - allowed_props
            if extra:
                model_errors.append(f"Disallowed additional properties: {sorted(extra)}")

            # Validate model_id pattern
            if "model_id" in model:
                if not isinstance(model["model_id"], str):
                    model_errors.append("model_id must be a string")
                elif not model_id_pattern.search(model["model_id"]):
                    pattern_str = schema_props.get("model_id", {}).get("pattern", "")
                    model_errors.append(f"model_id '{model['model_id']}' fails pattern '{pattern_str}'")

            # Validate source_commit pattern
            if "source_commit" in model:
                if not isinstance(model["source_commit"], str):
                    model_errors.append("source_commit must be a string")
                elif not source_commit_pattern.search(model["source_commit"]):
                    pattern_str = schema_props.get("source_commit", {}).get("pattern", "")
                    model_errors.append(f"source_commit '{model['source_commit']}' fails pattern '{pattern_str}'")

            # Validate evidence_class enum
            eclass = model.get("evidence_class")
            if eclass not in allowed_evidence_classes:
                model_errors.append(f"Invalid evidence_class '{eclass}'")
            if eclass == "empirically_calibrated_model":
                empirically_calibrated_count += 1

            # Validate nullable string fields
            for nullable_field in ["run_receipt", "parameter_set_id", "uncertainty"]:
                if nullable_field in model:
                    val = model[nullable_field]
                    if val is not None and not isinstance(val, str):
                        model_errors.append(f"{nullable_field} must be string or null")

            # Validate non-nullable string fields
            for str_field in ["model_version", "source_repository", "specification_version", "units", "coordinate_system", "known_limitations"]:
                if str_field in model:
                    val = model[str_field]
                    if not isinstance(val, str):
                        model_errors.append(f"{str_field} must be a string")

        if model_errors:
            failures += 1
            print(f"Model {idx:2d} ({mid:<45}) FAIL: {'; '.join(model_errors)}")
        else:
            print(f"Model {idx:2d} ({mid:<45}) PASS [class={model.get('evidence_class')}]")

    print("\n--- Epistemic Invariant Verification ---")
    print(f"empirically_calibrated_model count: {empirically_calibrated_count} (must be 0)")
    if empirically_calibrated_count != 0:
        print("FAIL: Invariant 3 violation! empirically_calibrated_model count must be 0.")
        failures += 1
    else:
        print("PASS: Invariant 3 satisfied (class empirically_calibrated_model is strictly empty).")

    print("\n=== Validation Summary ===")
    if failures == 0:
        print(f"SUCCESS: All {total_models} models strictly satisfy schema requirements! Exit code: 0")
        return True
    else:
        print(f"FAILURE: {failures} of {total_models} models failed schema validation! Exit code: 1")
        return False

if __name__ == "__main__":
    base_dir = Path(__file__).resolve().parent.parent
    s_path = str(base_dir / "data" / "models" / "schema.json")
    m_path = str(base_dir / "data" / "models" / "manifest.json")
    if len(sys.argv) > 1:
        s_path = sys.argv[1]
    if len(sys.argv) > 2:
        m_path = sys.argv[2]

    success = validate_models(s_path, m_path)
    sys.exit(0 if success else 1)
