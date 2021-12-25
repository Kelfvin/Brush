import re
import os
import src.ans as ans
import src.proc as proc
import src.fault_note as fault_note

while (True):
    os.system('cls' if os.name == 'nt' else 'clear')

    print('菜单:', '\n', '*' * 30)
    print('[1] 开始答题')
    print('[2] 导入题库')
    print('[3] 复习错题 {}'.format(fault_note.get_fault_note_numbers()))
    print('[4] 清空错题记录')
    print('[5] 清空刷题进度记录')
    print('[q] 退出')
    print('\n', '*' * 30, '\n')

    model = input('请输入你的选择[1|2|3|q]:')

    if (model == '1'):
        ans.ans()

    elif (model == '2'):
        proc.proc()

    elif (model == '3'):
        fault_note.review_fault()

    elif (model == '4'):
        os.system('cls' if os.name == 'nt' else 'clear')
        confirm = input('确认要清空错题记录吗？[Y/N]:')
        if (confirm == 'y' or confirm == 'Y'):
            file = open('./fault_note/fault_note', 'wb')
            file.close()
            print('已清空错题记录！')

    elif (model == '5'):
        os.system('cls' if os.name == 'nt' else 'clear')
        confirm = input('确认要清空刷题进度记录吗？[Y/N]:')
        if (confirm == 'y' or confirm == 'Y'):
            file_list = os.listdir('./trace/')
            for i in file_list:
                os.remove('./trace/{}'.format(i))
            print('已清空刷题进度记录！')

        else:
            print('取消成功！')

        input('输入任意键继续...')

    elif (model == 'q' or model == 'Q'):
        exit()
