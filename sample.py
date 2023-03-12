import orgorapython
with open('../sample_org/sample1.org', 'r') as file:
    data = file.read()
print(data)
html = orgorapython.parse_string(data)
print(html)
