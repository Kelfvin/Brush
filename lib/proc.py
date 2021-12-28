# coding=utf-8
import re
import os
import json


def proc(base_path,subject_name,out_file_name):
    os.system('cls' if os.name == 'nt' else 'clear')
    subject = {}
    subject['name'] = f'{subject_name}'

    file_list = os.listdir(f'{base_path}')
    subject['sections'] = {}
    for i in range(0, 100):
        try:
            file_in = open(f'{base_path}d{i}' + '.in',
                            encoding='utf-8',
                            mode='rt')         

            subject['sections'][f'第{i}章'] = {}
            subject['sections'][f'第{i}章']['problems'] ={}
            


            content = file_in.read()

            content = re.sub(r'我的答案:.*? (?P<c_asw>正确答案: [\s\S]*?)\n[\s\S]*?分',
                                '\g<c_asw>\n', content)
            content = re.sub(r'作业详情', '', content)
            content = re.sub(r'题量.*r?\n', '', content)
            content = re.sub(r'作答时间.*\n', '', content)

            content = re.sub(r'.+([一]\. [\s\S]*)', '', content)

            content = re.sub(r'，*\s*[0-9]+\.*[0-9]*分', '', content)
            content = re.sub(r'最终成绩.*\n', '', content)
            content = re.sub(r'作答记录.*\n', '', content)
            content = re.sub(r'答案解析.*\n', '', content)
            content = content.strip()
            content = re.sub(r'^.+\n', '# ' + '第{}章\n'.format(i), content)
            content = re.sub(r'\n', '\n\n', content)
            content = re.sub(r'(?P<sec_t>[一二三]\. .+?\n)', '## ' + '\g<sec_t>',
                                content)
            content = re.sub(r'(?P<th_t>[0-9]+\. .+?\n)', '### ' + '\g<th_t>',
                                content)

            content = re.sub(r'###', '*****\n###', content)
            content = re.sub(r'$', '\n*****', content)
            title = re.findall(r'# (.*)\n', content)[0]

            problems = re.findall(r'### (?P<problems>.*?)\n', content)
            options = re.findall(r'### .*?\n(?P<options>[\s\S]*?)\*\*\*\*\*',
                                    content)

            
            for index in range(0, len(problems)):

                type = re.findall(r'\((?P<type>\S+)\)', problems[index])[0]
                key = re.findall(r'正确答案: (.+?)\n', options[index])[0]

                options[index] = re.sub(r'正确答案.*\n', '', options[index])

                option = options[index]


                subject['sections'][f'第{i}章']['problems'][problems[index]]={}

                subject['sections'][f'第{i}章']['problems'][problems[index]]['key']=key
                subject['sections'][f'第{i}章']['problems'][problems[index]]['option']=option
                subject['sections'][f'第{i}章']['problems'][problems[index]]['type']=type
        except FileNotFoundError:
            continue

    with open(f'./data/bank/{out_file_name}',mode='wt',encoding='utf-8') as file_out:
        json.dump(subject,file_out,ensure_ascii=False,indent=4)

    print("处理完成！")
    input("输入任意键继续...")


def proc_m(base_path,subject_name,out_file_name):
    os.system('cls' if os.name == 'nt' else 'clear')
    subject = {}
    subject['name'] = f'{subject_name}'

    file_list = os.listdir(f'{base_path}')
    subject['sections'] = {}
    for i in range(0, 100):
        try:
            file_in = open(f'{base_path}d{i}' + '.in',
                            encoding='utf-8',
                            mode='rt')         

            subject['sections'][f'第{i}章'] = {}
            subject['sections'][f'第{i}章']['problems'] ={}
            


            content = file_in.read()


            content = re.sub(r'###', '*****\n###', content)
            content = re.sub(r'$', '\n*****', content)
            title = re.findall(r'# (.*)\n', content)[0]

            problems = re.findall(r'### (?P<problems>.*?)\n', content)
            options = re.findall(r'### .*?\n(?P<options>[\s\S]*?)\*\*\*\*\*',
                                    content)

            
            for index in range(0, len(problems)):

                type = re.findall(r'\((?P<type>\S+)\)', problems[index])[0]
                key = re.findall(r'正确答案: (.+?)\n', options[index])[0]

                options[index] = re.sub(r'正确答案.*\n', '', options[index])

                option = options[index]


                subject['sections'][f'第{i}章']['problems'][problems[index]]={}

                subject['sections'][f'第{i}章']['problems'][problems[index]]['key']=key
                subject['sections'][f'第{i}章']['problems'][problems[index]]['option']=option
                subject['sections'][f'第{i}章']['problems'][problems[index]]['type']=type
        except FileNotFoundError:
            continue

    with open(f'./data/bank/{out_file_name}',mode='wt',encoding='utf-8') as file_out:
        json.dump(subject,file_out,ensure_ascii=False,indent=4)

    print("处理完成！")
    input("输入任意键继续...")

proc_m('./data/bank/military/','军事理论','military.json')