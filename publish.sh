#!/usr/bin/env bash
set -euo pipefail

# ---- config (override via env if you want) ----
OWNER=${OWNER:-2enter}              # GHCR owner/org (lowercase)
REPO=${REPO:-frontier-builder-tsps-ntpc}                  # image prefix (lowercase)
PLATFORM=${PLATFORM:-linux/arm64}   # match your VPS
TAG=${TAG:-$(git rev-parse --short HEAD 2>/dev/null || date +%s)}
BUILDER=${BUILDER:-xbuilder}
REG=ghcr.io

# optional: login if GHCR_PAT is set (PAT needs write:packages; enable SSO for org)
if [[ -n "${GHCR_PAT:-}" ]]; then
  echo "$GHCR_PAT" | docker login ghcr.io -u "${GHCR_USER:-$OWNER}" --password-stdin
fi

# ensure containerized buildx (needed for cache export)
if ! docker buildx inspect "$BUILDER" >/dev/null 2>&1; then
  docker buildx create --name "$BUILDER" --driver docker-container --use
else
  docker buildx use "$BUILDER"
fi
docker buildx inspect --bootstrap >/dev/null

build () {
  local img dockerfile context
    img="$REG/$OWNER/${REPO}-backend"
    dockerfile="Dockerfile"
    context="."

    echo "==> Building backend → $img:$TAG ($PLATFORM)"
    docker buildx build \
      --builder "$BUILDER" \
      --platform "$PLATFORM" \
      -f "$dockerfile" \
      --cache-from type=registry,ref="${img}:buildcache" \
      --cache-to type=registry,ref="${img}:buildcache,mode=max" \
      -t "${img}:${TAG}" \
      -t "${img}:latest" \
      --provenance=false \
      --sbom=false \
      --push \
      "$context"

    echo "✅ Pushed: ${img}:${TAG} and ${img}:latest"
}

build