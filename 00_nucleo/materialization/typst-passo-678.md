---
# P678 — Sonda: suporte a pacotes `@preview`

> **Passo:** 678
> **Data:** 2026-07-10
> **Foco:** Pacotes (`#import "@preview/nome:versao"`) estão confirmados como ausentes desde P531, nunca investigados a fundo. É provavelmente a funcionalidade que mais limita o uso prático do cristalino hoje — grande parte dos documentos Typst reais da comunidade dependem de pacotes. Este passo é só sonda: mapear o mecanismo real do vanilla (onde os pacotes vêm, como são resolvidos, cacheados, versionados) antes de qualquer código.
> **Tipo:** Sonda directa. Sem implementação neste passo.
> **Tamanho:** M para a sonda; a implementação, se avançar, é provavelmente L ou XL.
> **ADR-0108 EM VIGOR.** **ADR-0114 EM VIGOR** — funcionalidade nova, nunca tocada; sonda obrigatória.
> **Dependências:** P531 (confirmação original de ausência).

---

## Sonda

### Confirmar o mecanismo de resolução de pacotes no vanilla

```bash
grep -rn "preview\|PackageSpec\|@preview" lab/typst-original/crates/typst-kit/src/ lab/typst-original/crates/typst-library/src/ 2>/dev/null | head -30
```

Confirmar:
1. De onde os pacotes são descarregados (um registo online, um caminho local, ambos)?
2. Como são cacheados localmente (directório, formato)?
3. Como a versão é resolvida (`@preview/nome:1.2.3`, ranges de versão, `*`)?
4. O que acontece sem ligação à internet — falha, ou usa só o que já está em cache?

### Testar directamente com o vanilla

```bash
cat > /tmp/p678-pacote.typ <<'EOF'
#import "@preview/cetz:0.2.2": canvas, draw
EOF
lab/typst-original/target/release/typst compile /tmp/p678-pacote.typ /tmp/p678-vanilla.pdf
echo "Exit code: $?"
```

Confirmar se o vanilla local em quarentena consegue de facto descarregar e usar um pacote real, ou se está isolado de rede (o que mudaria a forma de testar isto).

### Confirmar o formato do cache local

```bash
find ~/.cache/typst -maxdepth 3 2>/dev/null
ls -la ~/.local/share/typst/packages 2>/dev/null
```

Localizar onde o vanilla guarda pacotes já descarregados, e o formato exacto (estrutura de directórios, ficheiros de manifesto).

### Confirmar o estado actual do cristalino

```bash
cat > /tmp/p678-pacote.typ <<'EOF'
#import "@preview/cetz:0.2.2": canvas, draw
EOF
./target/release/typst /tmp/p678-pacote.typ /tmp/p678-cristalino.pdf
echo "Exit code: $?"
```

Confirmar a mensagem de erro exacta hoje.

### Mapear o alcance do trabalho

Para suportar pacotes, confirmar o que é preciso, nível a nível:

1. **Parsing:** reconhecer `@preview/nome:versao` na sintaxe de `#import`/`#include` (distinto de caminhos de ficheiro locais).
2. **Resolução de rede:** descarregar do registo oficial (confirmar o URL/protocolo usado pelo vanilla).
3. **Cache local:** guardar pacotes descarregados, para não repetir o download.
4. **Resolução de versão:** interpretar `1.2.3`, ranges, ou a versão mais recente.
5. **Execução:** o pacote descarregado é código Typst normal — depois de resolvido para um caminho local, deve reaproveitar o mecanismo de `#import` já existente para ficheiros locais.

### Critério de fecho da sonda

- [ ] Mecanismo do vanilla mapeado (fonte dos pacotes, cache, resolução de versão).
- [ ] Testado directamente se o vanilla local consegue mesmo descarregar (confirma ou refuta acesso de rede no ambiente de testes).
- [ ] Formato do cache local confirmado.
- [ ] Estado actual do cristalino confirmado, com mensagem de erro exacta.
- [ ] Os cinco níveis do trabalho mapeados, com estimativa de tamanho para cada um.
- [ ] Confirmado quanto do mecanismo de `#import` já existente pode ser reaproveitado (nível 5), reduzindo o trabalho realmente novo aos níveis 1-4.

---

## Decisão

Este passo não implementa nada. Produz o mapa e uma proposta de divisão em passos menores, seguindo o mesmo padrão já usado para outras funcionalidades grandes nesta conversa (P614, P628).

---

## Critério de fecho do passo

- [ ] Sonda completa, com evidência directa (testes, não suposição).
- [ ] Mapa dos cinco níveis, com estimativa de tamanho para cada um.
- [ ] Proposta de divisão em passos menores.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p678.md`.
