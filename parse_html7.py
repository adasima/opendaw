from html.parser import HTMLParser
import sys

class MyHTMLParser(HTMLParser):
    def __init__(self):
        super().__init__()
        self.text_content = []
        self.in_h4 = False
        self.in_code_header = False

    def handle_starttag(self, tag, attrs):
        if tag == 'h4':
            for name, value in attrs:
                if name == 'class' and 'code-header' in value:
                    self.in_code_header = True
                    return

    def handle_endtag(self, tag):
        if tag == 'h4':
             self.in_code_header = False

    def handle_data(self, data):
        if self.in_code_header:
            print(data.strip(), end=" ")

def parse_file(filename):
    print(f"\n--- {filename} ---")
    with open(filename, 'r', encoding='utf-8') as f:
        content = f.read()
    parser = MyHTMLParser()
    parser.feed(content)
    print("\n")

parse_file('target/doc/timestretch/engine/source/struct.SourceProducer.html')
parse_file('target/doc/timestretch/engine/graph/struct.EngineProcessor.html')
parse_file('target/doc/timestretch/engine/control/struct.EngineController.html')
