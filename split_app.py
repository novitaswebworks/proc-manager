import re

with open('src/app/mod.rs', 'r') as f:
    lines = f.readlines()

def extract_block(start_marker, end_marker):
    start_idx = -1
    end_idx = -1
    for i, line in enumerate(lines):
        if start_marker in line and start_idx == -1:
            start_idx = i
        if end_marker in line and start_idx != -1 and i > start_idx:
            # simple brace matching
            open_braces = 0
            for j in range(start_idx, len(lines)):
                open_braces += lines[j].count('{') - lines[j].count('}')
                if open_braces == 0:
                    end_idx = j
                    break
            break
    return start_idx, end_idx

start, end = extract_block("match self.active_screen {", "return Ok(())")
print(start, end)
