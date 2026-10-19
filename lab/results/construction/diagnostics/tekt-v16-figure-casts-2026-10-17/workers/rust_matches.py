import re

def mask(text):
    out=list(text);i=0
    while i<len(text):
        if text.startswith('//',i):
            j=text.find('\n',i)
            if j<0:j=len(text)
            for k in range(i,j):out[k]=' '
            i=j
        elif text[i]=='"':
            j=i+1
            while j<len(text):
                if text[j]=='\\':j+=2;continue
                if text[j]=='"':j+=1;break
                j+=1
            for k in range(i,j):out[k]=' '
            i=j
        else:i+=1
    return ''.join(out)

def block(text,marker,start=0):
    pos=text.index(marker,start);a=text.index('{',pos);m=mask(text);depth=1;j=a+1
    while depth:
        if m[j]=='{':depth+=1
        elif m[j]=='}':depth-=1
        j+=1
    return a,j,text[a+1:j-1]

def arms(body):
    m=mask(body);i=0;result=[]
    while i<len(body):
        while i<len(body) and ((m[i].isspace() and body[i]!='"') or m[i]==','):i+=1
        if i==len(body):break
        start=i;depth=0
        while i<len(body):
            if m[i] in '([{':depth+=1
            elif m[i] in ')]}':depth-=1
            if depth==0 and m.startswith('=>',i):break
            i+=1
        assert i<len(body),body[start:];pattern=body[start:i].strip();i+=2
        while m[i].isspace():i+=1
        a=i
        if m[i]=='{':
            depth=1;i+=1
            while depth:
                if m[i]=='{':depth+=1
                elif m[i]=='}':depth-=1
                i+=1
        else:
            depth=0
            while i<len(body):
                if m[i] in '([{':depth+=1
                elif m[i] in ')]}':depth-=1
                if m[i]==',' and depth==0:break
                i+=1
        result.append((pattern,body[a:i].strip().rstrip(',')))
    return result

def variants(source,name):
    body=source.split('pub enum '+name+' {',1)[1].split('\n}',1)[0];result=[]
    for line in body.splitlines():
        m=re.match(r'^    (\w+)(\(.*\))?,\s*$',line)
        if m:
            n,args=m.groups();arity=2 if n=='LocatedContent' else 1 if args else 0;result.append((n,arity))
    return result
