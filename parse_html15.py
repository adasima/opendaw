import re

def parse_file(filename):
    print(f"\n--- {filename} ---")
    with open(filename, 'r', encoding='utf-8') as f:
        content = f.read()
    matches = re.findall(r'<pre class="rust item-decl"><code>(.*?)</code></pre>', content, flags=re.DOTALL)
    for m in matches:
        print(re.sub(r'<[^>]*>', '', m).replace('&lt;', '<').replace('&gt;', '>').strip())

parse_file('target/doc/timestretch/core/types/enum.QualityMode.html')
