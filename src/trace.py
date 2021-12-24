import os
import pickle
import re


def store_trace(section, trace):
    file_out = open('./trace/' + section + '.trace', 'wb')
    pickle.dump(trace,file_out)
    file_out.close()


def get_trace(section):
    if (os.path.isfile('./trace/' + section + '.trace') ):
        file_in = open('./trace/' + section + '.trace', 'rb')
        content = file_in.read()
        if(len(content)):
            return pickle.loads(content)
        else:
            return 0

    else:
        return 0


def get_process(section):
    file = open('./bank/{}.md'.format(section),'rt',encoding='utf-8')
    content = file.read()
    total =  len(re.findall(r'### ',content))

    file_name = './trace/{}.trace'.format(section)
    if(os.path.isfile(file_name)):
        file_in = open(file_name,'rb')
        content = file_in.read()
        trace = get_trace(section)
        return '({}/{})'.format(trace,total)

    else:
        return '(0/{})'.format(total)
        