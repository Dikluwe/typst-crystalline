---
# P681 — Pacotes `@preview` só offline (P-β de P678)

> **Passo:** 681
> **Data:** 2026-07-10
> **Foco:** P678 mapeou o formato da cache local do vanilla (`~/.cache/typst/packages/{namespace}/{name}/{version}/`, com `typst.toml` e `entrypoint`). P679/P680 implementaram e verificaram `#import` de ficheiros locais. Este passo liga as duas coisas: resolver `#import "@preview/nome:versao": ...` para um ficheiro local já presente na cache do sistema, sem tocar em rede — o nível 5b+3 de P678.
> **Tipo:** Sonda mínima + Implementação.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P678 (mapa do formato de cache), P679/P680 (`#import` de ficheiros locais, a reaproveitar depois da resolução).

---

## Sonda mínima

### Confirmar o estado da cache local no ambiente actual

```bash
find ~/.cache/typst/packages -maxdepth 3 2>/dev/null
find ~/.local/share/typst/packages -maxdepth 3 2>/dev/null
```

Confirmar que `cetz/0.2.2` e `tidy/0.3.0` (descarregados durante a sonda de P678) continuam presentes, e ler o `typst.toml` de um deles:

```bash
cat ~/.cache/typst/packages/preview/cetz/0.2.2/typst.toml
```

### Confirmar o parsing de `PackageSpec` já existente no cristalino

```bash
grep -n "struct PackageSpec\|impl.*PackageSpec\|FromStr" 01_core/src/entities/package_spec.rs
```

Confirmar que `PackageSpec::from_str("@preview/cetz:0.2.2")` já funciona (P678 confirmou isto como "Feito" no nível 1) — testar directamente com um teste unitário mínimo antes de prosseguir, não assumir.

### Critério de fecho da sonda mínima

- [ ] Cache local confirmada presente e legível.
- [ ] `PackageSpec::from_str` confirmado a funcionar com um teste directo.

---

## Implementação

### 1. Resolução de `PackageSpec` para caminho local

Nova função (local a decidir, provavelmente em `03_infra`, dado que envolve acesso ao sistema de ficheiros fora do projecto): dado um `PackageSpec` com `namespace: "preview"`, procurar em `~/.cache/typst/packages/preview/{name}/{version}/` (e, se aplicável, na data dir também, seguindo a ordem de prioridade já confirmada por P678 — data dir primeiro, depois cache).

### 2. Parse do manifesto `typst.toml`

Ler o campo `entrypoint` do manifesto, para saber qual ficheiro dentro do directório do pacote é o ponto de entrada.

### 3. Ligação a `eval_module_import`

Em `eval_module_import` (`01_core/src/rules/eval/modules.rs`), o braço que hoje devolve "import de pacotes ainda não é suportado" passa a:
1. Fazer parse do `PackageSpec`.
2. Resolver para o caminho local (passo 1).
3. Ler o manifesto e obter o `entrypoint` (passo 2).
4. A partir daqui, reaproveitar exactamente o mesmo caminho já usado para ficheiros locais (P679) — registar o ficheiro, avaliar em módulo isolado, ligar bindings.

### 4. Erro claro quando o pacote não está em cache

Se o pacote pedido não estiver presente localmente (nem na data dir, nem na cache): erro claro, distinto do erro genérico de "pacotes não suportados" — algo como "pacote '@preview/nome:versao' não encontrado na cache local; download ainda não implementado (ver P-γ)".

### Critério de fecho da implementação

- [ ] `#import "@preview/cetz:0.2.2": canvas, draw` funciona, usando o pacote já em cache.
- [ ] Manifesto `typst.toml` correctamente lido, `entrypoint` resolvido.
- [ ] Pacote não presente na cache produz erro claro, distinto de "não suportado".
- [ ] `#import` de ficheiros locais (P679/P680) sem regressão.

---

## Validação

```bash
cat > /tmp/p681-cetz.typ <<'EOF'
#import "@preview/cetz:0.2.2": canvas, draw
#canvas({
  draw.line((0,0), (1,1))
})
EOF
./target/release/typst /tmp/p681-cetz.typ /tmp/p681.pdf
mutool draw -o /tmp/p681.png -r 150 /tmp/p681.pdf
```

Comparar visualmente com o resultado do vanilla no mesmo documento (já confirmado a funcionar por P678).

```bash
cat > /tmp/p681-nao-cacheado.typ <<'EOF'
#import "@preview/pacote-que-nao-existe:9.9.9": foo
EOF
./target/release/typst /tmp/p681-nao-cacheado.typ /tmp/p681-erro.pdf
echo "Exit code: $?"
```

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda mínima completa.
- [ ] Pacote real (`cetz`) importado com sucesso a partir da cache local, testado visualmente contra o vanilla.
- [ ] Pacote ausente produz erro claro e distinto.
- [ ] `#import` de ficheiros locais sem regressão.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p681.md`, com hash do commit.

---

## Próximo passo

P-γ (nível 2 de P678): download e cache de pacotes não presentes localmente.
