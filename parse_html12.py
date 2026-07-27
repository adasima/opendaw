import re

def parse_file(filename):
    print(f"\n--- {filename} ---")
    with open(filename, 'r', encoding='utf-8') as f:
        content = f.read()
    matches = re.findall(r'<h4[^>]*>pub fn (.*?)</h4>', content, flags=re.DOTALL)
    for m in matches:
        print(re.sub(r'<[^>]*>', '', m).replace('\n', ' ').strip())

parse_file('target/doc/timestretch/fn.stretch.html')
parse_file('target/doc/timestretch/fn.pitch_shift.html')
parse_file('target/doc/timestretch/engine/struct.Engine.html')
