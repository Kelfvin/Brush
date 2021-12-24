import os
import pickle


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
