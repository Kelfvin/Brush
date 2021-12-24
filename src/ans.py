import os
import re
import pickle
import src.fault_note as fault_note
import src.trace as trace


def ans():

    file_list = os.listdir(r'./bank/')
    os.system('cls')
    print('题库列表：')

    n = 0 
    for name in file_list:
        section = re.findall(r'(.*)\.md',name)[0]
        print('[{}].'.format(n),section,trace.get_process(section))
        n+=1

    choice = input('\n请输入你要进行的章节选项(输入u返回上一级菜单,q退出)[u/q]：')

    if(choice == 'u' or choice == 'U'):
        return

    elif(choice == 'q' or choice == 'Q'):
        exit()

    else:
        file = open('./bank/'+file_list[int(choice)],mode='rt',encoding='utf-8')
        content = file.read()
        content = re.sub(r'###','*****\n###',content)
        content = re.sub(r'$','\n*****',content)
        title = re.findall(r'# (.*)\n',content)[0]

        problems = re.findall(r'### (?P<problems>.*?)\n',content)
        options = re.findall(r'### .*?\n(?P<options>[\s\S]*?)\*\*\*\*\*',content)
        section = re.findall(r'(.*)\.md',file_list[int(choice)])[0]


        # 查找记录，开始记录位置遍历题目
        for index in range(trace.get_trace(section),len(problems)):
            trace.store_trace(section,index)
            os.system('cls')
            print(title,'\n')
            print('({}/{})'.format(index+1,len(problems)))
            print()

            type = re.findall(r'\((?P<type>\S+)\)',problems[index])[0]
            key = re.findall(r'正确答案: (.+?)\n',options[index])[0]

            
            options[index]=re.sub(r'正确答案.*\n','',options[index])
            print(problems[index])
            print(options[index])

            answer = input('请输入你的答案(输入u返回上一级,q退出)：')
            if(answer=='Q' or answer == 'q'):
                exit()

            elif(answer=='u' or answer=='U'):
                return

            elif(answer == key):
                print('正确！')
                fault_note.rmove_fault_note(title)

            else:
                print('错误！  正确答案：',key)
                fault_note.add_fault_note(problems[index],options[index],key)


            input("输入任意键继续：")
            os.system('cls')

        trace.store_trace(section,0)
        choice = input('还需要继续吗？\n[任意键]继续\n[q]退出')

        if(choice =='q'):
            exit()
