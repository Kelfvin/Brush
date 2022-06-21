import re

with open('./data/bank/Marxism/期末单选题.txt',encoding='utf-8',mode='rt') as file:
    content = file.read()

    keys = re.findall(r'正确答案：(?P<key>.)[\s\S].?',content)

    content = re.sub(r'正确答案：(?P<key>.)[\s\S].?','',content)

    print(keys)

    itr = re.finditer(r'\(\)',content)

    
    index = 0
    for i in itr:
        print(index)
        i = keys[index]
        index +=1

    with open('out.txt',encoding='utf-8',mode='wt') as file_out:
        file_out.write(content)
