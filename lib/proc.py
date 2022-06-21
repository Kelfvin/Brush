# coding=utf-8
from distutils.filelist import findall
import re
import os
import json
from turtle import title


def proc(base_path,subject_name,out_file_name):
    os.system('cls' if os.name == 'nt' else 'clear')
    subject = {}
    subject['name'] = f'{subject_name}'

    file_list = os.listdir(f'{base_path}')
    subject['sections'] = []
    for i in range(0, 100):
        try:
            file_in = open(f'{base_path}d{i}' + '.in',
                            encoding='utf-8',
                            mode='rt')         
            
            section = {};
            section["name"] = f'第{i}章'
            section["progress"] = 0;
            section["problems"] = []



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


            problems = re.findall(r'### [0-9]+\. ?(?P<problems>.*?)\n', content)
            options = re.findall(r'### .*?\n(?P<options>[\s\S]*?)\*\*\*\*\*',
                                    content)
            
            
            for index in range(0, len(problems)):

                type = re.findall(r'\((?P<type>\S+)\)', problems[index])[0]
                key = re.findall(r'正确答案: (.+?)\n', options[index])[0]

                options[index] = re.sub(r'## .*?\n', '', options[index])

                options[index] = re.sub(r'正确答案.*\n', '', options[index])

                option = options[index]

                mProblem = {}
                mProblem["title"] = problems[index];

                if type == "判断题":
                    if key == "对":
                        mProblem["key"] = "A"

                    else:
                        mProblem["key"] = "B"

                else:
                    mProblem['key'] = key

                mProblem["type"] = type

               
                mOptions = re.findall(r'[A-Z]\. .+[\s\S]+?', option)
                mProblem["options"] = mOptions
                section['problems'].append(mProblem)
            
            subject['sections'].append(section);

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
    subject['sections'] = []
    for i in range(0, 100):
        try:
            file_in = open(f'{base_path}d{i}' + '.in',
                            encoding='utf-8',
                            mode='rt')         

            section = {};
            section["name"] = f'第{i}章'
            section["progress"] = 0;
            section["problems"] = []
            


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

                key = re.sub(r'[\[\'\,\]]','',key)
                key = key.replace(' ','')

                mProblem = {}
                mProblem["title"] = problems[index]

                if type == "判断题":
                    if key == "对":
                        mProblem["key"] = "A"

                    else:
                        mProblem["key"] = "B"

                else:
                    mProblem['key'] = key

                mProblem["type"] = type

               
                mOptions = re.findall(r'[A-Z]. \n*?.+?\n', options[index])
                mProblem["options"] = mOptions
                section['problems'].append(mProblem)
            
            subject['sections'].append(section);

        except FileNotFoundError:
            continue

    with open(f'./data/bank/{out_file_name}',mode='wt',encoding='utf-8') as file_out:
        json.dump(subject,file_out,ensure_ascii=False,indent=4)

    print("处理完成！")
    input("输入任意键继续...")

def proc_yjdhp(base_path,subject_name,out_file_name):
    os.system('cls' if os.name == 'nt' else 'clear')
    subject = {}
    subject['name'] = f'{subject_name}'

    file_list = os.listdir(f'{base_path}')
    subject['sections'] = []
    for i in range(0, 100):
        try:
            file_in = open(f'{base_path}d{i}' + '.in',
                            encoding='utf-8',
                            mode='rt')         
            
            section = {};
            section["name"] = f'第{i}章'
            section["progress"] = 0;    #做题的进度
            section["problems"] = []



            content = file_in.read()

            keys = re.findall(r'我的答案：(.+)\n?', content)


            optionsList = re.findall(r'[0-9]*?\n【[\s\S]*?】.*\n([\s\S]*?)我的答案',content)

            titles = re.findall(r'(【[\s\S]+?】.+?)\n',content)

            types = re.findall(r'【([\s\S]*?)】',content)

            for index in range(0,len(titles)):

                ProblemItem = {}

                ProblemItem['title'] = titles[index]

                ProblemItem['type'] = types[index];
                ProblemItem['key'] = keys[index];
                ProblemItem['options'] = re.findall(r'[A-Z]、\n*?.+?\n',optionsList[index])

                if ProblemItem['type'] == "判断题":
                    ProblemItem['options']=['A 正确','B 错误']
                    if ProblemItem['key'] == "√":
                        ProblemItem['key'] = "A"

                    else:
                        ProblemItem['key'] = "B"

                section['problems'].append(ProblemItem)

            subject['sections'].append(section)

        except FileNotFoundError:
            continue

    with open(f'./data/bank/{out_file_name}',mode='wt',encoding='utf-8') as file_out:
        json.dump(subject,file_out,ensure_ascii=False,indent=4)

    print("处理完成！")
    input("输入任意键继续...")


def proc_Marxism(base_path,subject_name,out_file_name):
    os.system('cls' if os.name == 'nt' else 'clear')
    subject = {}
    subject['name'] = f'{subject_name}'

    file_list = os.listdir(f'{base_path}')
    subject['sections'] = []
    nameList = ['期末单选题','期末多选题','期末判断题']
    for i in nameList:
        try:
            file_in = open(f'{base_path}{i}' + '.txt',
                            encoding='utf-8',
                            mode='rt')         
            
            section = {};
            section["name"] = f'{i}'
            section["progress"] = 0;    #做题的进度
            section["problems"] = []



            content = file_in.read()

            keys = re.findall(r'正确答案：(.+)\n?', content)


            optionsList = re.findall(r'[0-9]+? [\s\S]+?(A[\s\S]+?)正确答案',content)



            type = re.findall(r'期末(.+)',i)[0]

            if type != '判断题':
                titles = re.findall(r'([0-9][\s\S]+?)A',content)  

            else:
                 titles = re.findall(r'([0-9][\s\S]+?)正确答案',content)  

            for index in range(0,len(titles)):

                ProblemItem = {}

                ProblemItem['title'] = titles[index]

                ProblemItem['type'] = type;
                ProblemItem['key'] = keys[index];

                if type != '判断题':
                    ProblemItem['options'] = re.findall(r'[A-Z]、\n*?.+?\n',optionsList[index])

                if ProblemItem['type'] == "判断题":
                    ProblemItem['options']=['A 正确','B 错误']
                    if ProblemItem['key'] == "√":
                        ProblemItem['key'] = "A"

                    else:
                        ProblemItem['key'] = "B"

                section['problems'].append(ProblemItem)

            subject['sections'].append(section)

        except FileNotFoundError:
            continue

    with open(f'./data/bank/{out_file_name}',mode='wt',encoding='utf-8') as file_out:
        json.dump(subject,file_out,ensure_ascii=False,indent=4)

    print("处理完成！")
    input("输入任意键继续...")



proc_m('./data/bank/military/','军事理论','military.json')
proc('./data/bank/history/','近代史','history.json')
proc_yjdhp('./data/bank/yjdhp/','思修','yjdph.json')
proc_Marxism('./data/bank/Marxism/','马原','Marxism.json')


