#!/bin/bash

# List of dependencies to install
dependencies=(
  "@mui/x-date-pickers"
  "@mui/material"
  "@mui/icons-material"
  "react-clock"
  "dayjs"
  "react-time-picker"
  "@emotion/react"
  "@emotion/styled"
)

for dep in "${dependencies[@]}"; do
  echo "Installing $dep..."
  npm install "$dep"
done

echo "Deps installed."
