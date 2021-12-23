import re
import os

file_list = os.listdir(r'./bank/')

while(True):
    os.system('cls')
    print('题库列表：')

    n = 0
    for name in file_list:
        section = re.findall(r'(.*)\.md',name)[0]
        print('[{}].'.format(n),section)
        n+=1

    choice = input('\n请输入你要进行的章节选项：')


    file = open('./bank/'+file_list[int(choice)],mode='rt',encoding='utf-8')
    content = file.read()
    content = re.sub(r'###','*****\n###',content)
    content = re.sub(r'$','\n*****',content)
    title = re.findall(r'# (.*)\n',content)[0]

    file_out = open('temp',mode='wt',encoding='utf-8')
    file_out.write(content)

    problems = re.findall(r'### (?P<problems>.*?)\n',content)
    options = re.findall(r'### .*?\n(?P<options>[\s\S]*?)\*\*\*\*\*',content)



    for index in range(0,len(problems)):
        os.system('cls')
        print(title,'\n')
        print('({}/{})'.format(index+1,1+len(problems)))
        print()

        type = re.findall(r'\((?P<type>\S+)\)',problems[index])[0]
        key = re.findall(r'正确答案: (.+?)\n',options[index])[0]

        
        options[index]=re.sub(r'正确答案.*\n','',options[index])
        print(problems[index])
        print(options[index])

        answer = input('请输入你的答案：')
        if(answer==key):
            print('正确！')

        else:
            print('正确答案：',key)

        temp = input("输入任意键继续：")
        os.system('cls')

    
    choice = input('还需要继续吗？\n[任意键]继续\n[q]退出')

    if(choice =='q'):
        exit()

    
