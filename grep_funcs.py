import re

def get_fns(filename):
    print(f"\n--- {filename} ---")
    with open(filename) as f:
        content = f.read()

    matches = re.findall(r'<h4 class="code-header">pub fn <a href="#method\.[a-zA-Z0-9_]+" class="fn">([a-zA-Z0-9_]+)</a>', content)
    for match in matches:
        print(match)

get_fns('target/doc/timestretch/engine/source/struct.SourceProducer.html')
get_fns('target/doc/timestretch/engine/graph/struct.EngineProcessor.html')
get_fns('target/doc/timestretch/engine/control/struct.EngineController.html')
