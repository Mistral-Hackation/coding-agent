# Compliance Reviewer Knowledge Base

> Internal reference for the ComplianceReviewer agent when verifying European regulatory compliance.

## 🎯 Role Summary

The ComplianceReviewer ensures that 3D-modeled industrial equipment designs comply with European Union regulations, particularly CE marking, ATEX, and PED requirements.

---

## 📚 Key European Regulations

### CE Marking

**What is CE?**
- "Conformité Européenne" (European Conformity)
- Mandatory marking for products sold in EU/EEA
- Indicates compliance with essential health, safety, and environmental requirements

**CE Marking Process:**
1. Identify applicable EU directives
2. Perform conformity assessment
3. Compile technical documentation
4. Draft EU Declaration of Conformity
5. Affix CE mark to product

**Key Directives for Industrial Equipment:**
| Directive | Application |
|-----------|-------------|
| Machinery (2006/42/EC) | Mechanical equipment |
| Low Voltage (2014/35/EU) | Electrical equipment 50-1000V AC |
| EMC (2014/30/EU) | Electromagnetic compatibility |
| PED (2014/68/EU) | Pressure equipment |
| ATEX (2014/34/EU) | Explosive atmospheres |

---

### ATEX Directive (2014/34/EU)

**Scope:** Equipment for potentially explosive atmospheres

**Explosive Atmosphere:** Mixture of air + gases/vapors/mists/dusts that can ignite

**Zone Classifications:**

| Zone | Gas/Vapor | Dust | Description |
|------|-----------|------|-------------|
| 0 / 20 | Zone 0 | Zone 20 | Explosive atmosphere present continuously |
| 1 / 21 | Zone 1 | Zone 21 | Explosive atmosphere likely during operation |
| 2 / 22 | Zone 2 | Zone 22 | Explosive atmosphere unlikely, brief if occurs |

**Equipment Categories:**
| Category | Suitable Zones | Protection Level |
|----------|----------------|------------------|
| 1 | 0, 1, 2 (20, 21, 22) | Very high |
| 2 | 1, 2 (21, 22) | High |
| 3 | 2 (22) | Normal |

**ATEX Marking Example:**
```
CE 0081 ⬡ II 2G Ex d IIB T4 Gb
```
- CE: Conformity mark
- 0081: Notified Body number
- ⬡: ATEX symbol (hexagon with Ex)
- II: Equipment Group II (non-mining)
- 2G: Category 2, Gas
- Ex d: Flameproof enclosure
- IIB: Gas group
- T4: Temperature class (≤135°C)
- Gb: Equipment Protection Level

---

### Pressure Equipment Directive (PED 2014/68/EU)

**Scope:** Stationary pressure equipment with Maximum Allowable Pressure > 0.5 bar

**Covered Equipment:**
- Pressure vessels
- Piping
- Safety accessories
- Pressure accessories
- Assemblies

**Hazard Categories:**

| Category | Risk Level | Assessment |
|----------|------------|------------|
| I | Lowest | Self-certification |
| II | Low | Type examination or QA |
| III | Medium | Type + Production QA |
| IV | Highest | Full Notified Body involvement |

**Essential Safety Requirements:**
- [ ] Design for adequate strength
- [ ] Provisions for safe inspection
- [ ] Means of draining and venting
- [ ] Corrosion protection measures
- [ ] Safe handling provisions
- [ ] Protection against overpressure

---

## 🔧 Compliance Verification Checklist

### General CE Requirements
- [ ] All applicable directives identified
- [ ] Technical file documentation adequate
- [ ] Risk assessment performed
- [ ] Notified Body involvement (if required)
- [ ] Declaration of Conformity prepared
- [ ] CE mark properly affixed

### ATEX Specific
- [ ] Zone classification determined
- [ ] Equipment category appropriate
- [ ] Ignition sources controlled
- [ ] Temperature class adequate
- [ ] Ingress protection (IP) rating sufficient
- [ ] Ex marking complete and accurate

### PED Specific
- [ ] Pressure category determined (I-IV)
- [ ] Material certificates available
- [ ] Weld procedures qualified
- [ ] NDT requirements met
- [ ] Pressure testing completed
- [ ] Safety devices specified

---

## 📐 Temperature Classes (ATEX)

| Class | Max Surface Temp | Ignition Temp of Gas |
|-------|------------------|---------------------|
| T1 | 450°C | > 450°C |
| T2 | 300°C | > 300°C |
| T3 | 200°C | > 200°C |
| T4 | 135°C | > 135°C |
| T5 | 100°C | > 100°C |
| T6 | 85°C | > 85°C |

---

## ⚠️ Compliance Red Flags

1. **Missing Markings**: No CE, ATEX, or PED markings specified
2. **Wrong Category**: Equipment rated below required zone/pressure
3. **Material Issues**: Uncertified materials for pressure service
4. **Temperature Violation**: Surface temp exceeds class limits
5. **Documentation Gaps**: Missing declarations or certificates

---

## 📋 Approval Criteria

**APPROVE** if:
- All applicable directives addressed
- Correct categories and classifications
- Proper markings specified
- Design meets essential requirements

**REVISE** if:
- Regulatory requirements not addressed
- Incorrect classifications
- Missing safety features
- Documentation gaps identified
