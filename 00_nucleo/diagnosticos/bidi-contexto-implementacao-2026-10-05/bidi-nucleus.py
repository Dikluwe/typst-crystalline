from pathlib import Path
import hashlib,re,json
n=Path('00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml');n.write_text('''tekt = 1
kind = "nucleus"
id = "paragraph-bidi-origin"
title = "Identidade original de fragmentos bidi"

[[claims]]
id = "exact-origin"
level = "must"
statement = "Carriers textuais somente recebem BidiSpan quando o texto corresponde exatamente ao range UTF-8 de entrada lógica integral certificada. None é obrigatório na ausência dessa prova; não inferir identidade por strings iguais, estilo ou posição."

[[claims]]
id = "preservation"
level = "must"
statement = "Clones, translações, envelopes e rollback conservam a origem. Splits exatos conservam subspans. Merge somente conserva span único para ranges contíguos no mesmo input e segmento; de outro modo conservar fragmentos separados."

[[claims]]
id = "auxiliary-isolation"
level = "must-not"
statement = "Controles virtuais da representação Unicode auxiliar não entram em Content, texto de carriers, glifos ou ToUnicode. Controles autorais não podem ser apagados como se fossem virtuais."

[[claims]]
id = "local-lifetime"
level = "must"
statement = "O input e a resolução permanecem locais à compilação. Não adicionar retenção entre edições, globals mutáveis, unsafe ou contexto na chave TextStyle. Unicode bidi permanece L3."
''')
pin=hashlib.sha256(n.read_bytes()+b'\0TEKT-NUCLEUS-DEPS-V1\0').hexdigest()
for p in Path('00_nucleo/prompts').rglob('*.md'):
 s=p.read_text()
 if 'Migração do carrier bidi ratificado (2026-10-05)' in s:
  s=re.sub(r'## Migração do carrier bidi ratificado \(2026-10-05\)\n.*?(?=\n## |\Z)', '## Migração do carrier bidi ratificado (2026-10-05)\n\nMedição anterior à decisão: o inventário `00_nucleo/diagnosticos/bidi-contexto-2026-10-05/inventory.json` identifica as referências desta unidade e os owners 1:1. O contrato do manifesto 1ae7513e foi confirmado; nesta unidade ajustar constructors não certificados com None e patterns de leitura com `..`, preservando os observáveis de linguagem existentes. Transformações obedecem ao núcleo pinado de origem; sem ampliar fase/API.\n',s,flags=re.S)
 else:
  if not any(x in s for x in ['Transporte lógico textual — contrato ratificado','Contexto integral bidi ratificado — 2026-10-05'] ) and p.name not in ['bidi_context.md','bidi_origin.md']:continue
 if f'- {n} sha256:' not in s:
  if 'Núcleos Tekt:\n' in s:s=s.replace('Núcleos Tekt:\n',f'Núcleos Tekt:\n- {n} sha256:{pin}\n',1)
  else:s+=f'\nNúcleos Tekt:\n- {n} sha256:{pin}\n'
 p.write_text(s)
print(pin)
