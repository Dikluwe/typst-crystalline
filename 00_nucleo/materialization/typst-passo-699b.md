---
# P699b — Confirmar com documento `.typ` real, não só testes unitários

> **Passo:** 699b
> **Data:** 2026-07-10
> **Foco:** P699 implementou `plugin()` → `Module` real e testou extensivamente com testes unitários Rust (chamadas directas a `call_plugin`, `PluginFunc`, etc.), mas explicitamente adiou o teste de compilar um documento `.typ` real através do pipeline completo (parse → eval → field access → chamada). Esta conversa já aprendeu, com P679, que bugs de sintaxe só aparecem ao compilar documentos reais — testes unitários que constroem valores directamente em Rust não exercitam o parser nem a resolução de `field access`/chamada de função a partir de sintaxe Typst real. Este passo é leve: só confirma isso, não é um teste de regressão completo.
> **Tipo:** Verificação directa. Correcção se confirmado um problema.
> **Tamanho:** XS.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P699 (implementação e testes unitários já feitos), P679 (precedente do mesmo tipo de gap).

---

## Verificação

```bash
cd /tmp
python3 << 'PYEOF'
# Reconstrói o hello.wasm de P696/P698 (encoder à mão, sem toolchain)
def uleb(n):
    out=[]
    while True:
        b=n&0x7f; n>>=7
        out.append(b|0x80 if n else b)
        if not n: break
    return bytes(out)
def sleb(n):
    out=[]
    while True:
        b=n&0x7f; n>>=7
        done=(n==0 and (b&0x40)==0) or (n==-1 and (b&0x40)!=0)
        out.append(b if done else b|0x80)
        if done: break
    return bytes(out)
def vbytes(v): return uleb(len(v))+bytes(v)
def vec_raw(c): return uleb(len(c))+b"".join(c)
def section(i,p): return bytes([i])+uleb(len(p))+p
def name(s): b=s.encode(); return uleb(len(b))+b
I32=0x7f
def ft(p,r): return bytes([0x60])+vbytes([I32]*len(p))+vbytes([I32]*len(r))
t0,t1,t2=ft([I32],[]),ft([I32,I32],[]),ft([],[I32])
type_sec=section(1,vec_raw([t0,t1,t2]))
def imp(m,n,ti): return name(m)+name(n)+bytes([0])+uleb(ti)
import_sec=section(2,vec_raw([imp("typst_env","wasm_minimal_protocol_write_args_to_buffer",0),
                              imp("typst_env","wasm_minimal_protocol_send_result_to_host",1)]))
func_sec=section(3,vbytes([2]))
mem_sec=section(5,vbytes([0])+uleb(1))
def em(n,i): return name(n)+bytes([2])+uleb(i)
def ef(n,i): return name(n)+bytes([0])+uleb(i)
export_sec=section(7,vec_raw([em("memory",0),ef("hello",2)]))
body=(bytes([0x41])+sleb(0)+bytes([0x41])+sleb(5)+bytes([0x10])+uleb(1)+
      bytes([0x41])+sleb(0)+bytes([0x0b]))
code_sec=section(10,vec_raw([uleb(len(uleb(0)+body))+uleb(0)+body]))
off=bytes([0x41])+sleb(0)+bytes([0x0b])
data_sec=section(11,vec_raw([uleb(0)+off+vbytes(list(b"hello"))]))
wasm=b"\x00asm\x01\x00\x00\x00"+type_sec+import_sec+func_sec+mem_sec+export_sec+code_sec+data_sec
open("hello.wasm","wb").write(wasm)
PYEOF
```

```bash
cat > /tmp/p699b-plugin.typ <<'EOF'
#let p = plugin("hello.wasm")
#str(p.hello())
EOF
cd /tmp
lab/typst-original/target/release/typst compile p699b-plugin.typ p699b-vanilla.pdf 2>&1 || echo "AVISO: rede pode ser necessária para wasmi runtime real; se o binário vanilla local já corre plugins, isto deve funcionar offline"
pdftotext p699b-vanilla.pdf - 2>/dev/null
./target/release/typst p699b-plugin.typ p699b-cristalino.pdf
echo "Exit code: $?"
pdftotext p699b-cristalino.pdf -
```

Confirmar: o documento real compila (não um teste unitário, o binário a processar texto Typst de verdade), e o texto extraído é `hello`, igual ao vanilla.

### Testar também o `#import` de plugin com sintaxe real

```bash
cat > /tmp/p699b-import.typ <<'EOF'
#import plugin("hello.wasm"): hello
#str(hello())
EOF
./target/release/typst /tmp/p699b-import.typ /tmp/p699b-import.pdf
pdftotext /tmp/p699b-import.pdf -
```

### Critério de fecho da verificação

- [ ] Documento real (`plugin(...)` + field access + chamada) compilado com sucesso através do binário, não só testes unitários Rust.
- [ ] Texto extraído confirmado igual ao vanilla.
- [ ] `#import plugin(...): item` testado com sintaxe real.

---

## Decisão

Se tudo funcionar: confirma que a integração está correcta ponta a ponta, não só nas peças isoladas — P699 fica validado com confiança maior.

Se alguma coisa falhar (por exemplo, o parser não reconhecer `plugin(...)` numa posição de expressão onde `field access` é depois aplicado, ou o despacho de chamada não encontrar o braço `FuncRepr::Plugin` a partir da sintaxe real): corrigir imediatamente — é exactamente o tipo de gap que só aparece testando o caminho real, como já aconteceu antes nesta conversa.

---

## Critério de fecho do passo

- [ ] Documento real testado, resultado confirmado ou problema encontrado e corrigido.
- [ ] `#import plugin(...): item` testado com sintaxe real.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p699b.md`, com o resultado exacto (texto extraído, comparação com vanilla).
