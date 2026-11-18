#!/usr/bin/env python3
"""Generate git-tree-colored.pdf. Requires Git, Graphviz, and pdftotext."""
import os
from pathlib import Path
os.chdir(Path(__file__).resolve().parent)
import subprocess,json,colorsys,heapq
from html import escape

def git(*args,input=None):
 return subprocess.run(['git',*args],input=input,text=True,check=True,capture_output=True).stdout
q=json.dumps
dead={l.split()[2] for l in git('fsck','--full','--no-reflogs','--unreachable').splitlines() if l.startswith('unreachable commit ')}
main=set(git('rev-list','--first-parent','main').split())
commits=set(git('rev-list','--all','--reflog').split())|dead
refs={}
for row in git('for-each-ref','--format=%(objectname)%09%(refname:short)','refs/heads','refs/remotes').splitlines():
 sha,name=row.split('\t')
 if name=='origin/HEAD': continue
 refs.setdefault(sha,[]).append(name)
records={}
for row in git('log','--stdin','--no-walk','--decorate=short','--format=%H%x09%P%x09%h%x09%s%x09%d',input='\n'.join(sorted(commits))+'\n').splitlines():
 sha,parents,short,msg,dec=row.split('\t',4)
 records[sha]=(parents.split(),short,msg,dec)
parents={p for row in records.values() for p in row[0]}
tips=set(refs) | (commits-parents) | {p for row in records.values() for p in row[0][1:]}
# Include annotated-tag and stash tips as well as named branch tips.
seeds=sorted(tips-main,key=lambda s: ('/'.join(refs.get(s,[])) or '~'+s))
owner={}
queue=[(0,i,s) for i,s in enumerate(seeds)]
heapq.heapify(queue)
while queue:
 distance,i,sha=heapq.heappop(queue)
 if sha in main or sha in owner: continue
 owner[sha]=i
 for parent in records[sha][0]:
  if parent in records: heapq.heappush(queue,(distance+1,i,parent))
assert all(s in main or s in owner for s in commits)
def color(i):
 h=(i*0.61803398875)%1
 def hexrgb(s,l):
  return '#'+''.join(f'{round(v*255):02x}' for v in colorsys.hls_to_rgb(h,l,s))
 return hexrgb(.62,.77),hexrgb(.72,.34)
def colors(sha):
 return ('#e8eaed','#80868b') if sha in main else color(owner[sha])
lines=['digraph history {','graph [rankdir=BT, bgcolor="white", label="RGEN\\nRust Terrain Generation in GIT living color", labelloc=t];','node [shape=box, style="rounded,filled", fontname="DejaVu Sans", fontsize=10, penwidth=1.6];']
for sha,(parents,short,msg,dec) in records.items():
 fill,border=colors(sha)
 names=refs.get(sha,[])
 if names:
  header='<BR/>'.join(('<FONT COLOR="#d4a000">★</FONT> ' if name=='origin/main' else '')+escape(name)+(' (HEAD)' if 'HEAD -&gt; '+escape(name) in escape(dec) else '') for name in names)
  label='<TABLE BORDER="0" CELLBORDER="0" CELLSPACING="0" CELLPADDING="6">'
  label+=f'<TR><TD><TABLE BORDER="1" CELLBORDER="0" CELLSPACING="0" CELLPADDING="6" STYLE="ROUNDED" COLOR="{border}" BGCOLOR="#ffffff"><TR><TD><FONT POINT-SIZE="12"><B>{header}</B></FONT></TD></TR></TABLE></TD></TR>'
  label+=f'<TR><TD><FONT POINT-SIZE="9">{escape(short)}</FONT><BR/>{escape(msg)}</TD></TR>'
  extra=[v.strip() for v in dec.strip().strip('()').split(',') if v.strip().startswith('tag:')]
  if extra: label+='<TR><TD>'+escape(', '.join(extra))+'</TD></TR>'
  label+='</TABLE>'
  lines.append(f'{q(sha)} [label=<{label}>, margin="0.06,0.06", fillcolor="{fill}", color="{border}"];')
 else:
  label=short+'\n'+msg
  if dec.strip(): label+='\n'+dec.strip()
  lines.append(f'{q(sha)} [label={q(label)}, fillcolor="{fill}", color="{border}"];')
 for parent in parents:
  lines.append(f'{q(parent)} -> {q(sha)} [color="{border}"];')
lines.append('}')
subprocess.run(['dot','-Tpdf','-o','git-tree-colored.pdf'],input='\n'.join(lines),text=True,check=True)
text=subprocess.check_output(['pdftotext','-layout','git-tree-colored.pdf','-'],text=True)
assert '[unreachable]' not in text
assert all(name in text for names in refs.values() for name in names)
print(f'Overwrote git-tree-colored.pdf: {len(records)} commits; {len(seeds)} distinct non-main branch/history tips. All branch labels verified; unreachable labels removed.')
