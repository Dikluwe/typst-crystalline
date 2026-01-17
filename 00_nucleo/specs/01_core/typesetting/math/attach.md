# 🧬 Crystal Facet: math/attach.rs

> **Crystal Face**: The Satellite Positioning — Orbital Displacement Geometry.

---

## 💎 Facet DNA

$$
\text{attach} : (\text{nucleus}, \text{sub}^?, \text{sup}^?) \to \text{Frame}
$$

**attach.rs** positions **satellite quanta** (sub/superscripts) in orbital relationship to their nucleus.

---

## Prescriptive Axioms

### Axiom I: Satellite Gravity Axiom

$$
\Delta y_{sub} = f(\mu_{nucleus}, \tau_{space})
$$
$$
\Delta y_{sup} = g(\mu_{nucleus}, \tau_{space})
$$

**Satellite Gravity Axiom**: The vertical displacement of a sub/superscript is a **function of**:
1. The **magnitude of the nucleus** (height, depth)
2. The **tension constant** of the current mathematical space

$$
\text{shift}_{sup} = \max(\mu_{min}, h_{nucleus} - \text{depth}_{sup})
$$

---

### Axiom II: Scale Reduction

$$
\text{size}_{satellite} = \text{size}_{nucleus} \times \rho
$$

Satellites undergo **scale reduction** — their size is proportionally smaller than the nucleus.

---

### Axiom III: Horizontal Positioning

$$
x_{sub} = x_{nucleus} + w_{nucleus}
$$
$$
x_{sup} = x_{nucleus} + w_{nucleus} + \text{italicCorrection}
$$

Satellites attach at the **trailing edge** of the nucleus, with italic correction for superscripts.

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    SATELLITE ORBITS                             │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│                        sup                                      │
│                      ┌───┐                                      │
│                      │ 2 │  ← Δy_sup (gravity pull upward)      │
│                      └───┘                                      │
│   Nucleus: ████████ x                                           │
│                      ┌───┐                                      │
│                      │ i │  ← Δy_sub (gravity pull downward)    │
│                      └───┘                                      │
│                        sub                                      │
│                                                                 │
│   Displacement = f(nucleus magnitude, space tension)            │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│       THE SATELLITE POSITIONING (math/attach.rs)         │
├──────────────────────────────────────────────────────────┤
│  Role: Orbital displacement geometry                     │
│                                                          │
│  Laws:                                                   │
│    ✓ Satellite Gravity Axiom — f(magnitude, tension)     │
│    ✓ Scale Reduction — proportional sizing               │
│    ✓ Horizontal Positioning — trailing edge attachment   │
└──────────────────────────────────────────────────────────┘
```
