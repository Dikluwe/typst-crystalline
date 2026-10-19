from pathlib import Path
import gzip,json,datetime,hashlib
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));r=json.load(gzip.open(out/'before-expanded-measurement.json.gz'));assert len(r['cases'])==260
common='''
## Domínio de figura e medidas — 2026-10-17

### Medição anterior à decisão

00_nucleo/diagnosticos/tekt-v16-figure-casts-2026-10-17/before.json.gz e
before-expanded-measurement.json.gz fixam HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b,
working tree não commitado, UTC, status/diffstat e fontes/L0/config/binários.
São 130 pares CLI. O pin é 9f2b6e8715237cb086899a42873660fe744622e8,
binário 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
figure_image.rs:68-105 silencia body inválido como Empty, caption inválida como
nome do tipo e kind inválido como inferência. O pin rejeita essas entradas;
body precede caption, que precede kind, mesmo se a ordem nomeada for invertida.
kind aceita string/auto/função de elemento, mas não função arbitrária/With.
O pin valida FigureKind em model/figure.rs:666-681 como Element ou EcoString.

casting.rs:24-53 conserva Rel na largura de block, mas extract_length reduz
Relative à parte absoluta e Ratio a zero. h/box aceitam números onde o pin
rejeita; .fields() também é incompleto. Isto mede dívida real, não prova
intenção upstream nem autoriza trocar percentuais por erro. Os carriers
públicos Spacing::Absolute(Length), BoxedElem.width:Option<Length> e
FigureElem.kind:Option<String> não transportam todos os dados de linguagem.
Essa evolução pública e eventual síntese de figure exigem proposta ADR-0127;
não as simular com nomes, valores neutros ou deslocamento informal de fase.
'''
figure='''
### Decisão exclusiva — rejeições de native_figure

Corrigir body: Content/LocatedContent, Str e None são conteúdo (None vazio);
outros valores erram expected content, found {vanilla_type_name}. Ausência
continua erro, agora missing argument: body. Caption aceita None (ausência),
Content/LocatedContent e Str; inválido erra expected content or none, found
{tipo}. Preservar a extração existente de FigureCaption.body. Não mudar
native_figure_caption, imagens nem o transporte de metadados de caption.

Kind: Str preserva o nome; Auto conserva o estado automático histórico.
Func é válida somente se reconhecida como elemento pela tabela já existente
native_element_fields ou Func::element_name. Não comparar Func pelo nome.
Função arbitrária/With erra expected element. Outros Value, incluindo None,
erram expected function, string, or auto, found {tipo}; não caem em inferência.
Funções de elemento válidas ainda conservam o caminho histórico de inferência
até o contrato público próprio; não alegar kind:table equivalente a string
ou identidade tipada correta nesta fatia. Essa dívida permanece aberta.

Diagnósticos de tipo apontam para value_span da ocorrência correspondente:
primeiro body posicional, último caption/kind nomeado conforme a view de Args.
Fallback: span da ocorrência, depois args.span; não buscar origem por igualdade
Value. Ordem de validação body→caption→kind, sem reavaliar expressões nem
alterar Args/ocorrências. Ausência usa diagnóstico detached como as nativas
existentes para o caller atribuir origem. Não expandir argumentos suportados
nem mudar prioridade de extras/nomes desconhecidos nesta correção.

Não encobrir infer_kind_from_body: seu V16 e suas limitações de recursão e
fase ficam abertos para a proposta de síntese/identidade pública. Não listar
containers como folhas neutras apenas para fazer o linter passar.
'''
casting='''
### Decisão exclusiva — cobertura fechada dos conversores

extract_length e extract_block_width classificam Value exaustivamente. Os
mesmos cinco casos Length/Float/Int/Relative/Ratio conservam seus RHS nesta
etapa. O grupo explícito das outras 34 variantes é coeso por ausência de cast
nesta projeção; retorna None para o caller validar, não um comprimento neutro.
Uma variante futura exige classificação no compilador. Preservar o contrato
atual de block, Fraction em seus owners e os adaptadores height/box. Não
restringir Float/Int ou percentuais globalmente, pois isso afeta callers de
contratos diferentes. A truncagem já medida permanece dívida para a evolução
do carrier público; os dois V16 estruturais não são prova de paridade completa.
'''
accept='''
### Aceitação

Correção interna/paridade em fluxo contínuo ADR-0127, sem assinatura pública,
default deliberado de produto, compatibilidade Rust ou fase novos. L0 primeiro,
testes funcionais RED para rejeições reais e controles válidos; GREEN exige
mensagens e âncoras causais e precedência sem avaliação repetida. Testes dos
casts controlam comprimentos afins, block relativo e rejeições de famílias;
não cristalizar truncagem como semântica final. Sondas estruturais extraem
padrões produtivos para provar variante futura rejeitada nos dois matches;
payloads stub/RHS apagado não constituem equivalência funcional.

Repetir os 130 pares, distinguir correções intencionais, controles e dívidas
remanescentes, sem declarar todo JSON/fields/reflexão equivalente. Inspecionar
os consumers dos carriers e registrar inventário antes de propor API. V15/V26
válido antes de resselo apenas destes dois owners. Build, suíte workspace,
Clippy -D warnings/-D unfulfilled_lint_expectations, fmt, Rustdoc -D warnings
com quatro índices, linhagem estrita/reversa, diff e SARIF diferencial.
12 GiB rígido, high 10 GiB, swap zero, jobs=1; execução pesada sequencial.
Sem release, merge ou commit/push do Typst. RG1 permanece independente.
'''
for source,owner in json.load(open(out/'scope.json'))['owners'].items():
 p=Path(owner);s=p.read_text();s+=common+(figure if source.endswith('figure_image.rs') else casting)+accept;p.write_text(s)
(out/'prescription.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),owners={p:hashlib.sha256(Path(p).read_bytes()).hexdigest() for p in json.load(open(out/'scope.json'))['owners'].values()}),indent=2)+'\n')
