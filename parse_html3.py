from html.parser import HTMLParser
import sys

class MyHTMLParser(HTMLParser):
    def __init__(self):
        super().__init__()
        self.in_pre = False
        self.code = []
        self.text_content = []

    def handle_starttag(self, tag, attrs):
        if tag == 'pre':
            self.in_pre = True
        elif tag == 'code' and self.in_pre:
            pass # We extract text inside

    def handle_endtag(self, tag):
        if tag == 'pre':
            self.in_pre = False

    def handle_data(self, data):
        self.text_content.append(data)
        if self.in_pre:
            self.code.append(data)

def parse_file(filename):
    with open(filename, 'r', encoding='utf-8') as f:
        content = f.read()
    parser = MyHTMLParser()
    parser.feed(content)

    print("\n--- CODE SNIPPETS ---")
    print("".join(parser.code[:5000]))

parse_file('target/doc/timestretch/engine/struct.EngineHandles.html')
parse_file('target/doc/timestretch/engine/struct.EngineConfig.html')
