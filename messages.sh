#!/usr/bin/env bash
# -L. brings in this directory to the jq path
#
jq -L. 'include "jq_defs"; payload' trace.jsonl
