from html.parser import HTMLParser
import sys

class MyHTMLParser(HTMLParser):
    def __init__(self):
        super().__init__()
        self.text_content = []
        self.in_fn = False
        self.in_pre = False
        self.code = []

    def handle_starttag(self, tag, attrs):
        if tag == 'pre':
             self.in_pre = True
        if tag == 'h4' and ('class', 'method') in attrs:
             self.in_fn = True
        elif tag == 'code' and ('class', 'code-header') in attrs:
             pass #self.in_fn = True

    def handle_endtag(self, tag):
        if tag == 'pre':
             self.in_pre = False
        if tag == 'h4':
             self.in_fn = False

    def handle_data(self, data):
        if self.in_fn:
            print(data.strip(), end=" ")
        if self.in_pre:
            self.code.append(data)

def parse_file(filename):
    print(f"\n--- {filename} ---")
    with open(filename, 'r', encoding='utf-8') as f:
        content = f.read()
    parser = MyHTMLParser()
    parser.feed(content)
    print("\n--- CODE ---")
    print("".join(parser.code))
    print("\n")

parse_file('target/doc/timestretch/engine/graph/struct.EngineProcessor.html')
