from html.parser import HTMLParser
import sys

class MyHTMLParser(HTMLParser):
    def __init__(self):
        super().__init__()
        self.in_pre = False
        self.code = []

    def handle_starttag(self, tag, attrs):
        if tag == 'pre':
            self.in_pre = True

    def handle_endtag(self, tag):
        if tag == 'pre':
            self.in_pre = False

    def handle_data(self, data):
        if self.in_pre:
            self.code.append(data)

def parse_file(filename):
    try:
        with open(filename, 'r', encoding='utf-8') as f:
            content = f.read()
        parser = MyHTMLParser()
        parser.feed(content)
        print("\n--- CODE SNIPPETS ---")
        print("".join(parser.code[:5000]))
    except:
        print(f"Error reading {filename}")

parse_file('target/doc/timestretch/engine/index.html')
parse_file('target/doc/timestretch/engine/struct.EngineController.html')
parse_file('target/doc/timestretch/engine/struct.EngineProcessor.html')
parse_file('target/doc/timestretch/engine/struct.SourceProducer.html')
