# styler
Tool to parse keybinding file and generate the binds necessary for inkscape-manager.
A keybind either applies a style or rebinds to another key. Rebinds can be done directly in inkscape.

# Config structure
A style consists of a list of key:value pairs. You can only modify one thing with a keybind, e.g. red or black dashstrokes.
That means that style for each keybind consists of one empty key followed by a series of default settings. e.g:
```
style: stroke; stroke-width:2.6, stroke-dasharray:15.8;...
```
`stroke` is the style that will be modified by pressing a second keybind, which would result in something like:
```
style: stroke:#000000; stroke-width:2.6, stroke-dasharray:15.8;...
```

The valid options are exemplified below. A key must either have a `style` or a `rebind_to`.
```
keybinds:
- key: a
  style: stroke;stroke-width:2.64566929;stroke-dasharray:15.87401575,2.64566929;stroke-opacity:1;
- key: w
  rebind_to: x
```

# Svg style structure
```
<?xml version="1.0" encoding="UTF-8" standalone="no"?><svg>\n<defs/><g></g><inkscape:clipboard style="fill:#000;" />\n</svg>
```
