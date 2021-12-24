import os
import re
import pickle


def solve_no_note():
    if(not os.path.isfile('./fault_note/fault_note')):
        file = open('./fault_note/fault_note','wb')
        file.close()

def add_fault_note(title, options, key):
    solve_no_note()
    file_in = open('./fault_note/fault_note', mode='rb')
    content = file_in.read()
    fault_dic = {}
    if (len(content)):
        fault_dic = pickle.loads(content)

    if (title not in fault_dic):
        temp = {'options': options, 'key': key}
        fault_dic[title] = temp

        file_in.close()

        file_out = open('./fault_note/fault_note', mode='wb')
        pickle.dump(fault_dic,file_out)
        file_out.close()

def rmove_fault_note(title):
    solve_no_note()
    file_in = open('./fault_note/fault_note', mode='rb')
    content = file_in.read()
    fault_dic = pickle.loads(content)

    if(title in fault_dic):
        del fault_dic[title]
        file_in.close()

        file_out = open('./fault_note/fault_note', mode='wb')
        pickle.dump(fault_dic,file_out)
        file_out.close()    
    
def review_fault():
    os.system('cls')
    solve_no_note()
    file_in = open('./fault_note/fault_note', mode='rb')
    content = file_in.read()
    fault_dic = {}
    if (len(content)):
        fault_dic = pickle.loads(content)

    times = 0
    for title,opt_key in fault_dic.items():
        times+=1
        options = opt_key['options']
        key = opt_key['key']

        os.system('cls')
        print('错题回顾')
        print('({}/{})'.format(times,1+len(fault_dic)))
        print(title)
        print(opt_key['options'],'\n')

        answer = input('请输入你的答案(输入u返回上一级,q退出)：')
        if(answer=='q' or answer == 'Q'):
            exit()

        elif(answer=='u'or answer=='U'):
            return

        elif(answer==key):
            print('正确！')
            rmove_fault_note(title)

        else:
            print('错误！  正确答案：',key)
            add_fault_note(title,options,key)

        input("输入任意键继续：")
        os.system('cls')

        
    choice = input('错题做完了！\n[任意键]退出\n[u]返回主菜单\n请输入你的选择[*|q]:')

    if(choice =='u'):
        return

    else:
        exit()
        