# styler
Tool to parse keybinding file and generate the binds necessary for inkscape-manager.
A keybind either applies a style or rebinds to another key. Rebinds can be done directly in inkscape.

# Config structure
```
keybinds:
- key: a
  style: fill
  value: "#000000"
- key: f
  style: opacity
  value: "0.5"
- key: w
  rebind_to: x
```

# Svg style structure
```
<?xml version="1.0" encoding="UTF-8" standalone="no"?><svg>\n<defs/><g></g><inkscape:clipboard style="fill:#000;" />\n</svg>
```
