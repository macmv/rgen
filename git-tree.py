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
# Suppress selected stash snapshots from the graph while preserving stash data.
hidden={
 '58b3fa835ef34793c1777cfc820a66a7b76886a5', 'd33715b8026ef7c93d66151119e2dd96d11ca9bb',
 '2be93bf111f0118c017a396a92ed0dac012c6c78', 'e23ea5729e3ed30dbb3fea3b4377297a96f871f6',
 '4f27fabdb1944584f667ccc163b07f1a24020cc2', '4210b62b231e14764b38c52c3c4ac95e78572152',
 '8bd09dfd11e08a34ea409b3f3e2b791653f39828',
 'dc03c06932ec333c29702071ee77bb085560f8d6', 'c0c55e2943fab9e2256d4ad696cc78c6aad718f5',
 '403c20db7873b6fd55a98ad3243dbcd7d94e237d', 'cd09cb1c5b1fd8efedda607ff2a84b286a131be5',
 'dd212f688ab620f3d30dcab7d31bad826b48790e', 'f363bdaf122306aa29344ca89e50dae885166690',
 '608af60bda6c687c5e283c0942b60683fbb1f725', '6efe410a834be3c9ba7a8eb561397b89a912ffb1',
 '172147fa0439bf24918147105e172f577908d271',
 # Old, incorrect local merge placement; the branch ref was moved back.
 '0917a49a117550296bbc6ffcb2e632005dbe0640',
}
commits=(set(git('rev-list','--all','--reflog').split())|dead)-hidden
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

# Track branch-specific ancestry for shared edges. Keep at most two colored
# strokes per edge so overlapping histories remain legible in the PDF.
branch_seeds=[(i,sha) for i,sha in enumerate(seeds) if sha in refs]
edge_branches={}
for branch_i,tip in branch_seeds:
 seen=set()
 pending=[tip]
 while pending:
  sha=pending.pop()
  if sha in main or sha in seen or sha not in records: continue
  seen.add(sha)
  for parent in records[sha][0]:
   if parent in main or parent not in records: continue
   edge_branches.setdefault((parent,sha),set()).add(branch_i)
   pending.append(parent)

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
  shared=edge_branches.get((parent,sha),set())
  if len(shared)>1:
   own=owner.get(sha)
   selected=sorted(shared)
   if own in shared:
    selected=[own,next(i for i in selected if i != own)]
   else:
    selected=selected[:2]
   for branch_i in selected:
    lines.append(f'{q(parent)} -> {q(sha)} [color="{color(branch_i)[1]}", penwidth=1.6];')
  else:
   lines.append(f'{q(parent)} -> {q(sha)} [color="{border}"];')
lines.append('}')
subprocess.run(['dot','-Tpdf','-o','git-tree-colored.pdf'],input='\n'.join(lines),text=True,check=True)
text=subprocess.check_output(['pdftotext','-layout','git-tree-colored.pdf','-'],text=True)
assert '[unreachable]' not in text
assert all(name in text for names in refs.values() for name in names)
print(f'Overwrote git-tree-colored.pdf: {len(records)} commits; {len(seeds)} distinct non-main branch/history tips. All branch labels verified; unreachable labels removed.')
