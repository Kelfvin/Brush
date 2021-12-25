# coding=utf-8
import re
import os


def proc():
    os.system('cls' if os.name == 'nt' else 'clear')
    file_list = os.listdir('.\\raw\\')
    for i in range(0, len(file_list)):
        file_in = open('.\\raw\\d{}'.format(i) + '.in', encoding='utf-8', mode='rt')
        content = file_in.read()
        file_out = open('.\\bank\\第{}章'.format(i) + '.md', mode='wt', encoding='utf-8')
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
        content = re.sub(r'(?P<sec_t>[一二三]\. .+?\n)', '## ' + '\g<sec_t>', content)
        content = re.sub(r'(?P<th_t>[0-9]+\. .+?\n)', '### ' + '\g<th_t>',
                         content)

        file_out.write(content)

        file_in.close()
        file_out.close()

    print("处理完成！")
    input("输入任意键继续...")
