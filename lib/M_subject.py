import json
import pickle
import lib.M_frac as M_frac
import lib.M_menu as M_menu
import lib.M_fault_note as M_fault_note
import os


def from_trace_to_subject(file_name):
    base_path = './data/trace/'
    with open(f'{base_path}{file_name}',mode='rb') as file_in:
        content = file_in.read()
        return pickle.loads(content)

class Problem:
    def __init__(self,subject_name,json_in) -> None:
        (title,content) = json_in
        self.title = title
        self.type = content['type']
        self.options = content ['option']
        self.key = content['key']
        self.subject_name = subject_name
  
    def judge(self,answer):
        answer = answer.replace('1','A')
        answer = answer.replace('2','B')
        answer = answer.replace('3','C')
        answer = answer.replace('4','D')
        answer = answer.replace('5','E')
        answer = answer.replace('6','F')

        answer = answer.replace(' ','')
        if self.type == '判断题':
            if answer == 'A':
                answer = '对'

            elif answer == 'B':
                answer = '错'

        return True if self.key ==answer  else False


    def show(self):
        print(self.title)
        print(self.options)

    def get_key(self):
        return self.key

    def add_fault_note(self):
        M_fault_note.add_fault_note(self.subject_name,self.title,self.type,self.options,self.key)

    def remove_fault(self):
        M_fault_note.rmove_fault_note(self.subject_name,self.title)

        
class Section:
    def __init__(self,subject_name,json_in) -> None:
        (name,problems) = json_in
        self.name = name
        self.problems = []
        self.trace = 0

        for i in problems['problems'].items():
            self.problems.append(Problem(subject_name,i))


    def study(self):
        length = len(self.problems)
        for index in range(self.trace,length):
            M_frac.clear()
            item = self.problems[index]
            print(f'{self.name}  ({self.trace}/{length})')
            item.show()
            M_menu.show_exit_return_menu()
            answer = M_frac.get_message()
            if (answer == 'Q'):
                return 0

            elif(answer == 'U'):
                return 1

            elif item.judge(answer):
                print('正确！')


            else:
                key = item.get_key()
                print(f'错误！ 正确答案{key}')
                item.add_fault_note()

            self.trace+=1

            print('[*] 任意键进入下一题')
            message = M_frac.get_message()
            if(message.lower() == 'U'):
                return 1

            elif(message.lower() == 'Q'):
                return 0

        M_frac.clear()
        print('题目已经做完了！')
        M_menu.show_exit_return_menu()
        message = M_frac.get_message()
        if(message.lower() == 'U'):
            return 1

        elif(message.lower() == 'Q'):
            return 0

    def get_name(self):
        return self.name

    def get_trace(self):
        return self.trace

    def get_problem_num(self):
        return len(self.problems)
        
    def reset(self):
        self.trace = 0

class Subject:
    def __init__(self,json_in) -> None:
        self.name = json_in['name']
        self.sections = []
        for i in json_in['sections'].items():
            self.sections.append(Section(self.name,i))

    def get_section_list_trace(self):
        dic = {}
        for i in self.sections:
            section_name = i.get_name()
            trace = i.get_trace()
            len = i.get_problem_num()
            section_trace = f'{trace}/{len}'
            dic[section_name] = section_trace
        

        return dic
        

    def from_trace(file_name):
        base_path = './data/trace/'
        with open(f'{base_path}{file_name}',mode='rb') as file_in:
            content = file_in.read()
            return pickle.loads(content)

    def to_trace(self):
        base_path = './data/trace/'
        with open(f'{base_path}{self.name}',mode='wb') as file_out:
            pickle.dump(self,file_out)
            return True

    def get_name(self):
        return self.name


    def study(self):
        while(True):
            dic = self.get_section_list_trace()
            M_menu.show_section_menu(dic,self.get_fault_num())
            message = M_frac.get_message()

            if message == 'U':
                return 1

            elif message== 'Q':
                return 0

            elif message == 'R':
                if not self.reset_section():
                    return 0

            elif message == 'D':
                if self.rm_fault_note():
                    return 1

                else:
                    return 0

            elif int(message)>=0 and int(message) < len(self.sections):
                feed_back = self.sections[int(message)].study()
                self.to_trace()
                if feed_back == 0:
                    return 0


            elif int(message) == len(self.sections):
                if not self.review_fault():
                    return 0
                
                
            else:
                M_frac.erro_in_notion()

    def review_fault(self):
        return M_fault_note.review_fault(self.name)

    def get_fault_num(self):
        return M_fault_note.get_fault_note_numbers(self.name)


    def reset_section(self):
        M_frac.clear()
        print('********重置章节指定章节的做题进度********\n')
        M_menu.show_section_menu(self.get_section_list_trace(),0,flag=0,cc=0,r=0)
        M_menu.show_exit_return_menu()
        message = M_frac.get_message()
        if message == 'U':
            return 1

        elif message== 'Q':
            return 0

        else:
            try:
                M_frac.clear()
                print('你确定要重置该章节的进度吗？')
                M_menu.show_confirm_menu()
                M_menu.show_exit_return_menu()
                confirm = M_frac.get_message()
                if confirm == 'Y':
                    self.sections[int(message)].reset()
                    input('重置成功！输入任意键继续...')
                return 1
            except:
                M_frac.erro_in_notion()
                return 1

    def rm_fault_note(self):
        M_frac.clear()
        print('你确定要删除错题记录吗？')
        M_menu.show_confirm_menu()
        M_menu.show_exit_return_menu()
        message = M_frac.get_message()
        if message == 'U':
            return 1

        elif message== 'Q':
            return 0

        elif message == 'Y':
            M_fault_note.reset_fault_note(self.name)
            input('删除成功！输入任意键继续...')
            return 1

        else:
            M_frac.erro_in_notion()
            return 1

    def get_total_problem_num(self):
        sum = 0
        for i in self.sections:
            sum+=i.get_problem_num()

        return sum

    def get_trace(self):
        # 返回科目所作的题目总数
        sum = 0
        for i in self.sections:
            sum+=i.get_trace()

        return sum
                

        



             