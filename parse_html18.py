import re

def parse_file(filename):
    print(f"\n--- {filename} ---")
    with open(filename, 'r', encoding='utf-8') as f:
        content = f.read()
    matches = re.findall(r'<h4 class="code-header">pub fn (.*?)</h4>', content, flags=re.DOTALL)
    for m in matches:
        print(re.sub(r'<[^>]*>', '', m).replace('\n', ' ').replace('&lt;', '<').replace('&gt;', '>').strip())

parse_file('target/doc/timestretch/engine/profiles/enum.EngineProfile.html')
