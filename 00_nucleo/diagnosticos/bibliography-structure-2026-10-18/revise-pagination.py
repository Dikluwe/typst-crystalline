from pathlib import Path
import json,re,hashlib,gzip
p=Path(__file__).resolve().parent;f=Path('00_nucleo/prompts/compiler/layout/bibliography.md');s=f.read_text();old='Lista com prefixos usa Grid técnico de duas colunas Auto, gutter relativo0.65em, sem inset/pintura/tags. Representar row-gutter par.spacing por tracks Fixed entre rows Auto e células vazias nos tracks de intervalo; não criar margem terminal. Envolver a grade no Block existente e chamar layout diretamente, sem eval/show de Grid.'
new='''Lista com prefixos mede somente os prefixos puros fornecidos pelo CSL, uma vez por ocorrência de lista, para obter a largura intrínseca máxima comum. Cada entrada usa seu próprio Block breakable, inset na borda inicial igual a max_prefix_width + 0.65em; corpo segue o fluxo normal quebrável. Prefixo é um Place técnico sem float, alinhado à borda inicial/top do referencial interno do Block com dx que cancela o inset (negativo à esquerda, positivo à direita), emitido antes do corpo e só na primeira região da entrada. Não criar Grid nem reexecutar show/CSL/callback. O owner de Block resolve as margens fracas par.spacing entre entradas e sua continuação; não inventar offset vertical. Os owners 00_nucleo/prompts/compiler/layout/place.md e 00_nucleo/prompts/compiler/layout/placement.md já definem ancoragem ao block_place_region interno. Containers são detalhes privados de render, sem ocorrências/tags/Location sintetizada.'''
assert old in s;s=s.replace(old,new);marker='### Responsabilidade exclusiva\n\nQuando há plano privado'
measure='''### Refutador de paginação antes da revisão

pagination-before.json.gz / pagination-grid.json.gz e grid-freeze.json.gz no
mesmo diagnóstico preservam o candidato Grid e a contraprova: IEEE em página
200×90pt/margem8 exige duas páginas no oracle e no Tekt anterior, mas o
candidato usa três, iniciando toda a entrada na página2 e OUTSIDE na3.
Gutter/coluna e separação em página auto estavam corretos (A. x29.924 e
segunda entrada y112.232), mas não bastam. grid/row_pagination.rs:23–31
admite overflow de linha e move row inteira; não é a quebra de um parágrafo
longo. Isto refuta a composição Grid desta subetapa e exige trocá-la antes
de fechar, sem enfraquecer o teste. A linguagem pede coluna estável e entrada
quebrável; não exige usar o mesmo Grid Rust do vanilla (ADR-0107/0109).
Nova hipótese usa Block/Place existentes; testes de primeira região, conteúdo
completo, fontes, alinhamento e caixas podem refutá-la, inclusive RTL.

### Responsabilidade exclusiva

Quando há plano privado'''
assert marker in s;s=s.replace(marker,measure);f.write_text(s)
n=Path('00_nucleo/prompts/_nuclei/layout/bibliography-plan.toml');t=n.read_text().replace('sem acrescentar ocorrências semânticas de Grid/Block','sem acrescentar ocorrências semânticas de Grid/Block/Place');n.write_text(t);pin=hashlib.sha256(n.read_bytes()+b'\0TEKT-NUCLEUS-DEPS-V1\0').hexdigest();d=json.loads((p/'prescription.json').read_text())
for rel in d['owners']:
 f=Path('00_nucleo/prompts')/rel;t=f.read_text();t=re.sub(r'(layout/bibliography-plan.toml sha256:)[a-f0-9]{64}',lambda m:m[1]+pin,t);f.write_text(t)
d['effective_pin']=pin;d['pagination_revision']='Grid rejected by bilateral pagination; Block/Place composition';(p/'prescription.json').write_text(json.dumps(d,indent=2)+'\n')
# Add the pagination refuter before replacing the candidate implementation.
f=Path('01_core/src/compiler/layout/bibliography.rs');s=f.read_text();i=s.rfind('\n}');s=s[:i]+'''
    #[test]
    fn a2_structure_long_entry_starts_in_remaining_region() {
        use crate::entities::layout_types::{PageDimension, PageMarginSpec};
        let bibliography = Content::Bibliography(Arc::new(BibliographyElem {
            entries: vec![BibEntry::new("entry", "Alpha, Ann", "FIRSTWORD Long Reference Title Must Wrap On Several Lines", 2020)],
            path: None, title: None, style: Some("ieee".into()), locale: None,
        }));
        let page = Content::page_run(Some(PageDimension::Length(200.0)), Some(PageDimension::Length(90.0)), Some(PageMarginSpec::uniform(8.0)), None, None,
            Content::sequence(vec![Content::text("CITE"), bibliography, Content::Parbreak, Content::text("OUTSIDE")]));
        let document = super::super::layout(&page);
        assert!(document.pages[0].plain_text().contains("FIRSTWORD"), "{:?}",document.pages.iter().map(|p|p.plain_text()).collect::<Vec<_>>());
        assert!(document.plain_text().contains("OUTSIDE"));
    }
'''+s[i:];f.write_text(s)
(p/'tests-pagination-before-code.json.gz').write_bytes(gzip.compress(json.dumps({str(f):s}).encode()))
