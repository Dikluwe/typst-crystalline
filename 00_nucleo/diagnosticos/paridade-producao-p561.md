# Relatório de Execução — P561: Ordem Visual do Texto RTL

**Passo:** 561  
**Data de execução:** 2026-07-04  
**Foco:** Confirmar qual descrição está correta: P484/P521 (existência de `bidi_runs` e correção de fronteira de cluster RTL) ou P560 (texto árabe aparece da esquerda para a direita porque o bidi layout ainda não está implementado).

---

## Arquivo de teste

```typst
#set text(lang: "ar", size: 40pt)
الكتاب على الطاولة
```

Frase: "الكتاب على الطاولة" — "O livro está na mesa" (árabe).  
Ordem lógica de leitura (RTL):

1. `الكتاب` — "o livro"
2. `على` — "sobre"
3. `الطاولة` — "a mesa"

---

## Comandos executados

```bash
# Compilação cristalina
./target/release/typst /tmp/p561-rtl-order.typ /tmp/p561-cristalino.pdf

# Compilação vanilla
lab/typst-original/target/release/typst compile /tmp/p561-rtl-order.typ /tmp/p561-vanilla.pdf

# Renderização para PNG
mutool draw -o /tmp/p561-cristalino.png -r 150 /tmp/p561-cristalino.pdf
mutool draw -o /tmp/p561-vanilla.png -r 150 /tmp/p561-vanilla.pdf
```

Ambas as compilações terminaram com `EXIT: 0`. Nenhum aviso ou erro de compilação foi emitido.

---

## Análise visual das imagens

### Vanilla (`/tmp/p561-vanilla.png`)

A linha está disposta corretamente para leitura RTL:

- A palavra mais à **direita** da linha é `الكتاب` ("o livro"), primeira palavra da frase.
- Seguindo para a esquerda, aparece `على` ("sobre").
- A palavra mais à **esquerda** da linha é `الطاولة` ("a mesa"), última palavra da frase.

**Conclusão vanilla:** a ordem visual das palavras na linha segue a direção RTL esperada para árabe. Dentro de cada palavra, as letras árabes estão ligadas correctamente (ex.: o lām-inicial de `الكتاب` liga-se ao kāf seguinte; as letras de `الطاولة` mantêm as formas médias/finais adequadas à posição na palavra).

### Cristalino (`/tmp/p561-cristalino.png`)

A linha está disposta na **ordem inversa**:

- A palavra mais à **esquerda** da linha é `الكتاب` ("o livro"), primeira palavra da frase.
- Seguindo para a direita, aparece `على` ("sobre").
- A palavra mais à **direita** da linha é `الطاولة` ("a mesa"), última palavra da frase.

**Conclusão cristalino:** as palavras aparecem na ordem LTR (da esquerda para a direita), o que é ilegível para um leitor de árabe. No entanto, dentro de cada palavra individual, o shaping está correto: as letras de `الكتاب` e `الطاولة` estão ligadas com as formas posicionais certas (inicial, média, final), tal como no vanilla.

---

## Reconciliação das descrições anteriores

| Aspeto | Estado no cristalino | Responsável pelos passos anteriores |
|--------|----------------------|-------------------------------------|
| **Shaping interno** de cada palavra árabe | Funciona correctamente | P484 (`bidi_runs`, `unicode-bidi`) e P521 (correcção de fronteira de cluster RTL) |
| **Ordem visual** das palavras na linha (layout de parágrafo RTL) | Está trocada (LTR) | Não implementado; descrição de P560 está correcta |

A contradição aparente dissolve-se quando se separa a camada de **shaping** (forma das letras dentro de um trecho) da camada de **layout de parágrafo** (posicionamento dos trechos ao longo da linha):

- **P484 e P521** introduziram/corrigiram mecanismos que permitem que cada trecho de texto árabe seja *shaped* correctamente — isto é, as letras dentro de cada palavra assumem as formas certas e ligam-se entre si. Isso está visível na imagem cristalina.
- **P560** estava a descrever a camada de **layout**: a informação de direccionalidade dos trechos ainda não é usada para ordenar visualmente as palavras na linha. O resultado é que cada palavra é desenhada correctamente por dentro, mas as palavras são colocadas da esquerda para a direita, como se a linha fosse LTR.

Corrigir algo na fronteira de cluster RTL (P521) faz sentido mesmo sem layout bidi completo, porque o shaping de trechos isolados ainda precisa de saber onde um trecho RTL começa e termina — por exemplo, para evitar que uma pontuação ou um espaço no limite quebre uma ligação árabe.

---

## Decisão

A descrição de **P560 está correcta**: o layout bidireccional de parágrafo ainda não está implementado no cristalino. A ordem visual das palavras num documento árabe está trocada (LTR), embora o shaping interno de cada palavra funcione.

Isto deve ser registado como um **item de trabalho próprio**, distinto do shaping já corrigido, com prioridade alta: um documento árabe legível exige que as palavras sejam ordenadas RTL na linha, não apenas que as letras dentro de cada palavra estejam bem ligadas.

---

## Critérios de fecho

- [x] Imagem comparada directamente, palavra por palavra.
- [x] Confirmado que a ordem das palavras na linha está trocada no cristalino (LTR em vez de RTL).
- [x] Confirmado que as letras dentro de cada palavra estão ligadas correctamente (shaping funcional) em ambos os casos.
- [x] Descrições de P484/P521 vs P560 reconciliadas: cada uma trata de uma camada diferente (shaping vs layout de parágrafo).
