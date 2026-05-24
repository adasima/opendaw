if git diff --name-only HEAD~1..HEAD 2>/dev/null | grep -qE '^(src/|Cargo\.(toml|lock))'; then
  echo "code_changed=true"
elif git diff --name-only origin/main...HEAD | grep -qE '^(src/|Cargo\.(toml|lock))'; then
  echo "code_changed=true"
else
  echo "code_changed=false"
fi
