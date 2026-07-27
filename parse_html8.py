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

    def handle_endtag(self, tag):
        if tag == 'pre':
             self.in_pre = False
        if tag == 'h4':
             self.in_fn = False

    def handle_data(self, data):
        if self.in_fn:
            print(data.strip(), end=" ")

def parse_file(filename):
    print(f"\n--- {filename} ---")
    try:
        with open(filename, 'r', encoding='utf-8') as f:
            content = f.read()
        parser = MyHTMLParser()
        parser.feed(content)
    except Exception as e:
        pass
    print("\n")

parse_file('target/doc/timestretch/index.html')
parse_file('target/doc/timestretch/fn.stretch_buffer.html')
parse_file('target/doc/timestretch/core/types/struct.AudioBuffer.html')
