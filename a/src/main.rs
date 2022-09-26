use std::collections::HashSet;
use std::io::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

fn main() {
    second();
}

type Set = HashSet<usize>;

fn second() {
    let (d, a, m, e) = dame();
    let (m, a, u) = maumau(a, m);
    let (h, a, l, m) = halma(a, m);
    let (m, u, e, h, l) = muehle(m, u, e, h, l);
    let (m, a, l, e, f, i, z) = malefiz(m, a, l, e);
    let (m, e, o, r, y) = memory(m, e);
    let (t, a, b, u) = tabu(a, u);
    let (q, u, i, z) = quiz(u, i, z);
    let (m, i, k, a, d, o) = mikado(m, i, a, d, o);
    let (u, n, o) = uno(u, o);
    let (c, l, u, e, d, o) = cluedo(l, u, e, d, o);
    let (b, r, i, d, g, e) = bridge(b, r, i, d, e);
    let (j, e, n, g, a) = jenga(e, n, g, a);
    let (s, c, h, a) = schach(c, h, a);
    let (l, u, e, g, n, m, a, x) = luegenmax(l, u, e, g, n, m, a);
    let (k, n, i, f, e, l) = kniffel(k, n, i, f, e, l);
    let (w, u, e, r, f, l, n) = wuerfeln(u, e, r, f, l, n);
    let (r, i, s, k, o) = risiko(r, i, s, k, o);
    let (c, a, n, s, t) = canasta(c, a, n, s, t);
    let (a, b, l, o, n, e) = abalone(a, b, l, o, n, e);
    let (d, o, m, i, n) = domino(d, o, m, i, n);
    let (s, c, r, a, b, l, e) = scrabble(s, c, r, a, b, l, e);
    let (a, c, t, i, v, y) = activity(a, c, t, i, y);
    let (u, b, o, n, g) = ubongo(u, b, o, n, g);
    let (m, o, n, p, l, y) = monopoly(m, o, n, l, y);

    let (g, y) = {
        let mut rg = HashSet::new();
        let mut ry = HashSet::new();

        for g in g.iter() {
            for y in y.iter() {
                if let(_, false) = g.overflowing_sub(*y) {
                    rg.insert(*g);
                    ry.insert(*y);
                }
            }
        }

        (rg, ry)
    };

    let (e, a, u) = {
        let mut re = HashSet::new();
        let mut ra = HashSet::new();
        let mut ru = HashSet::new();

        for e in e.iter() {
            for a in a.iter() {
                for u in u.iter() {
                    if e + a + u < 10 {
                        re.insert(*e);
                        ra.insert(*a);
                        ru.insert(*u);
                    }
                }
            }
        }

        (re, ra, ru)
    };

    let (s, f) = {
        let mut rs = HashSet::new();
        let mut rf = HashSet::new();

        for s in s.iter() {
            for f in f.iter() {
                if let(x, false) = s.overflowing_sub(*f) {
                    if x < 10 {
                        rs.insert(*s);
                        rf.insert(*f);
                    }
                }
            }
        }

        (rs, rf)
    };

    let (m, i) = {
        let mut rm = HashSet::new();
        let mut ri = HashSet::new();

        for m in m.iter() {
            for i in i.iter() {
                if m + i < 10 {
                    rm.insert(*m);
                    ri.insert(*i);
                }
            }
        }

        (rm, ri)
    };

    let (o, w) = {
        let mut ro = HashSet::new();
        let mut rw = HashSet::new();

        for o in o.iter() {
            for w in w.iter() {
                if let(x, false) = o.overflowing_sub(*w) {
                    if x < 10 {
                        ro.insert(*o);
                        rw.insert(*w);
                    }
                }
            }
        }

        (ro, rw)
    };

    let (q, z) = {
        let mut rq = HashSet::new();
        let mut rz = HashSet::new();

        for q in q.iter() {
            for z in z.iter() {
                if let(x, false) = q.overflowing_sub(*z) {
                    if x < 10 {
                        rq.insert(*q);
                        rz.insert(*z);
                    }
                }
            }
        }

        (rq, rz)
    };

    let (c, d) = {
        let mut rc = HashSet::new();
        let mut rd = HashSet::new();

        for c in c.iter() {
            for d in d.iter() {
                if let(x, false) = c.overflowing_sub(*d) {
                    if x < 10 {
                        rc.insert(*c);
                        rd.insert(*d);
                    }
                }
            }
        }

        (rc, rd)
    };

    let (k, r, u) = {
        let mut rk = HashSet::new();
        let mut rr = HashSet::new();
        let mut ru = HashSet::new();

        for k in k.iter() {
            for r in r.iter() {
                for u in u.iter() {
                    if let(x, false) = k.overflowing_sub(*r) {
                        if let(x, false) = x.overflowing_sub(*u) {
                            if x < 10 {
                                rk.insert(*k);
                                rr.insert(*r);
                                ru.insert(*u);
                            }
                        }
                    }
                }
            }
        }

        (rk, rr, ru)
    };

    let combinations = a.len() * b.len() * c.len() * d.len() * e.len() * f.len() *
        g.len() * h.len() * i.len() * j.len() * k.len() * l.len() * m.len() *
        n.len() * o.len() * p.len() * q.len() * r.len() * s.len() * t.len() *u.len() *v.len() *w.len() *
        x.len() * y.len() * z.len();
    println!("Total number of combinations:  {}", combinations);

    solution(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, combinations);
}

fn dame() -> (Set, Set, Set, Set) {
    let mut rd = HashSet::new();
    let mut ra = HashSet::new();
    let mut rm = HashSet::new();
    let mut re = HashSet::new();

    for a in 1..=26 {
        for d in 1..=26 {
            if HashSet::from([a, d]).len() != 2 {
                continue;
            }

            for m in 1..=26 {
                if HashSet::from([a, d, m]).len() != 3 {
                    continue;
                }

                for e in 1..=26 {
                    if HashSet::from([a, d, m, e]).len() != 4 {
                        continue;
                    }

                    if d + a + m + e == 11 {
                        rd.insert(d);
                        ra.insert(a);
                        rm.insert(m);
                        re.insert(e);
                    }
                }
            }
        }
    }

    (rd, ra, rm, re)
}

fn maumau(a: Set, m: Set) -> (Set, Set, Set) {
    let mut rm = HashSet::new();
    let mut ra = HashSet::new();
    let mut ru = HashSet::new();

    for m in m.iter() {
        for a in a.iter() {
            for u in 1..=26 {
                if HashSet::from([*m, *a, u]).len() != 3 {
                    continue;
                }

                if m + a + u + m + a + u == 14 {
                    rm.insert(*m);
                    ra.insert(*a);
                    ru.insert(u);
                }
            }
        }
    }
    (rm, ra, ru)
}

fn halma(a: Set, m: Set) -> (Set, Set, Set, Set) {
    let mut rh = HashSet::new();
    let mut ra = HashSet::new();
    let mut rl = HashSet::new();
    let mut rm = HashSet::new();

    for h in 1..=26 {
        for a in a.iter() {
            for l in 1..=26 {
                for m in m.iter() {
                    if HashSet::from([h, *a, l, *m]).len() != 4 {
                        continue;
                    }

                    if h + a + l + m + a == 19 {
                        rh.insert(h);
                        ra.insert(*a);
                        rl.insert(l);
                        rm.insert(*m);
                    }
                }
            }
        }
    }

    (rh, ra, rl, rm)
}

fn muehle(m: Set, u: Set, e: Set, h: Set, l: Set) -> (Set, Set, Set, Set, Set) {
    let mut rm = HashSet::new();
    let mut ru = HashSet::new();
    let mut re = HashSet::new();
    let mut rh = HashSet::new();
    let mut rl = HashSet::new();

    for m in m.iter() {
        for u in u.iter() {
            for e in e.iter() {
                for h in h.iter() {
                    for l in l.iter() {
                        if HashSet::from([*m, *u, *e, *h, *l]).len() != 5 {
                            continue;
                        }

                        if m + u + e + h + l + e == 25 {
                            rm.insert(*m);
                            ru.insert(*u);
                            re.insert(*e);
                            rh.insert(*h);
                            rl.insert(*l);
                        }
                    }
                }
            }
        }
    }

    (rm, ru, re, rh, rl)
}

fn malefiz(m: Set, a: Set, l: Set, e: Set) -> (Set, Set, Set, Set, Set, Set, Set) {
    let mut rm = HashSet::new();
    let mut ra = HashSet::new();
    let mut rl = HashSet::new();
    let mut re = HashSet::new();
    let mut rf = HashSet::new();
    let mut ri = HashSet::new();
    let mut rz = HashSet::new();

    for m in m.iter() {
        for a in a.iter() {
            for l in l.iter() {
                for e in e.iter() {
                    for f in 1..=26 {
                        for i in 1..=26 {
                            for z in 1..=26 {
                                if HashSet::from([*m, *a, *l, *e, f, i, z]).len() != 7 {
                                    continue;
                                }

                                if m + a + l + e + f + i + z == 50 {
                                    rm.insert(*m);
                                    ra.insert(*a);
                                    rl.insert(*l);
                                    re.insert(*e);
                                    rf.insert(f);
                                    ri.insert(i);
                                    rz.insert(z);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    (rm, ra, rl, re, rf, ri, rz)
}

fn memory(m: Set, e: Set) -> (Set, Set, Set, Set, Set) {
    let mut rm = HashSet::new();
    let mut re = HashSet::new();
    let mut ro = HashSet::new();
    let mut rr = HashSet::new();
    let mut ry = HashSet::new();

    for m in m.iter() {
        for e in e.iter() {
            for o in 1..=26 {
                for r in 1..=26 {
                    for y in 1..=26 {
                        if HashSet::from([*m, *e, o, r, y]).len() != 5 {
                            continue;
                        }

                        if m + e + m + o + r +y == 51 {
                            rm.insert(*m);
                            re.insert(*e);
                            ro.insert(o);
                            rr.insert(r);
                            ry.insert(y);
                        }
                    }
                }
            }
        }
    }

    (rm, re, ro, rr, ry)
}

fn tabu(a: Set, u: Set) -> (Set, Set, Set, Set) {
    let mut rt = HashSet::new();
    let mut ra = HashSet::new();
    let mut rb = HashSet::new();
    let mut ru = HashSet::new();

    for t in 1..=26 {
        for a in a.iter() {
            for b in 1..=26 {
                for u in u.iter() {
                    if HashSet::from([t, *a, b, *u]).len() != 4 {
                        continue;
                    }

                    if t + a + b + u == 52 {
                        rt.insert(t);
                        ra.insert(*a);
                        rb.insert(b);
                        ru.insert(*u);
                    }
                }
            }
        }
    }

    (rt, ra, rb, ru)
}

fn quiz(u: Set, i: Set, z: Set) -> (Set, Set, Set, Set) {
    let mut rq = HashSet::new();
    let mut ru = HashSet::new();
    let mut ri = HashSet::new();
    let mut rz = HashSet::new();

    for q in 1..=26 {
        for u in u.iter() {
            for i in i.iter() {
                for z in z.iter() {
                    if HashSet::from([q, *u, *i, *z]).len() != 4 {
                        continue;
                    }

                    if q + u + i + z == 54 {
                        rq.insert(q);
                        ru.insert(*u);
                        ri.insert(*i);
                        rz.insert(*z);
                    }
                }
            }
        }
    }

    (rq, ru, ri, rz)
}

fn mikado(m: Set, i: Set, a: Set, d: Set, o: Set) -> (Set, Set, Set, Set, Set, Set) {
    let mut rm = HashSet::new();
    let mut ri = HashSet::new();
    let mut rk = HashSet::new();
    let mut ra = HashSet::new();
    let mut rd = HashSet::new();
    let mut ro = HashSet::new();

    for m in m.iter() {
        for i in i.iter() {
            for k in 1..=26 {
                for a in a.iter() {
                    for d in d.iter() {
                        for o in o.iter() {
                            if HashSet::from([*m, *i, k, *a, *d, *o]).len() != 6 {
                                continue;
                            }

                            if m + i + k + a + d + o == 55 {
                                rm.insert(*m);
                                ri.insert(*i);
                                rk.insert(k);
                                ra.insert(*a);
                                rd.insert(*d);
                                ro.insert(*o);
                            }
                        }
                    }
                }
            }
        }
    }

    (rm, ri, rk, ra, rd, ro)
}

fn uno(u: Set, o: Set) -> (Set, Set, Set) {
    let mut ru = HashSet::new();
    let mut rn = HashSet::new();
    let mut ro = HashSet::new();

    for u in u.iter() {
        for n in 1..=26 {
            for o in o.iter() {
                if HashSet::from([*u, n, *o]).len() != 3 {
                    continue;
                }

                if u + n + o == 55 {
                    ru.insert(*u);
                    rn.insert(n);
                    ro.insert(*o);
                }
            }
        }
    }

    (ru, rn, ro)
}

fn cluedo(l: Set, u: Set, e: Set, d: Set, o: Set) -> (Set, Set, Set, Set, Set, Set) {
    let mut rc = HashSet::new();
    let mut rl = HashSet::new();
    let mut ru = HashSet::new();
    let mut re = HashSet::new();
    let mut rd = HashSet::new();
    let mut ro = HashSet::new();

    for c in 1..=26 {
        for l in l.iter() {
            for u in u.iter() {
                for e in e.iter() {
                    for d in d.iter() {
                        for o in o.iter() {
                            if HashSet::from([c, *l, *u, *e, *d, *o]).len() != 6 {
                                continue;
                            }

                            if c + l + u + e + d + o == 57 {
                                rc.insert(c);
                                rl.insert(*l);
                                ru.insert(*u);
                                re.insert(*e);
                                rd.insert(*d);
                                ro.insert(*o);
                            }
                        }
                    }
                }
            }
        }
    }

    (rc, rl, ru, re, rd, ro)
}

fn bridge(b: Set, r: Set, i: Set, d: Set, e: Set) -> (Set, Set, Set, Set, Set, Set) {
    let mut rb = HashSet::new();
    let mut rr = HashSet::new();
    let mut ri = HashSet::new();
    let mut rd = HashSet::new();
    let mut rg = HashSet::new();
    let mut re = HashSet::new();

    for b in b.iter() {
        for r in r.iter() {
            for i in i.iter() {
                for d in d.iter() {
                    for g in 1..=26 {
                        for e in e.iter() {
                            if HashSet::from([*b, *r, *i, *d, g, *e]).len() != 6 {
                                continue;
                            }

                            if b + r + i + d + g + e == 61 {
                                rb.insert(*b);
                                rr.insert(*r);
                                ri.insert(*i);
                                rd.insert(*d);
                                rg.insert(g);
                                re.insert(*e);
                            }
                        }
                    }
                }
            }
        }
    }

    (rb, rr, ri, rd, rg, re)
}

fn jenga(e: Set, n: Set, g: Set, a: Set) -> (Set, Set, Set, Set, Set) {
    let mut rj = HashSet::new();
    let mut re = HashSet::new();
    let mut rn = HashSet::new();
    let mut rg = HashSet::new();
    let mut ra = HashSet::new();

    for j in 1..=26 {
        for e in e.iter() {
            for n in n.iter() {
                for g in g.iter() {
                    for a in a.iter() {
                        if HashSet::from([j, *e, *n, *g, *a]).len() != 5 {
                            continue;
                        }

                        if j + e + n + g + a == 65 {
                            rj.insert(j);
                            re.insert(*e);
                            rn.insert(*n);
                            rg.insert(*g);
                            ra.insert(*a);
                        }
                    }
                }
            }
        }
    }

    (rj, re, rn, rg, ra)
}

fn schach(c: Set, h: Set, a: Set) -> (Set, Set, Set, Set) {
    let mut rs = HashSet::new();
    let mut rc = HashSet::new();
    let mut rh = HashSet::new();
    let mut ra = HashSet::new();

    for s in 1..=26 {
        for c in c.iter() {
            for h in h.iter() {
                for a in a.iter() {
                    if HashSet::from([s, *c, *h, *a]).len() != 4 {
                        continue;
                    }

                    if s + c + h + a + c + h == 65 {
                        rs.insert(s);
                        rc.insert(*c);
                        rh.insert(*h);
                        ra.insert(*a);
                    }
                }
            }
        }
    }

    (rs, rc, rh, ra)
}

fn luegenmax(l: Set, u: Set, e: Set, g: Set, n: Set, m: Set, a: Set) -> (Set, Set, Set, Set, Set, Set, Set, Set) {
    let mut rl = HashSet::new();
    let mut ru = HashSet::new();
    let mut re = HashSet::new();
    let mut rg = HashSet::new();
    let mut rn = HashSet::new();
    let mut rm = HashSet::new();
    let mut ra = HashSet::new();
    let mut rx = HashSet::new();

    for l in l.iter() {
        for u in u.iter() {
            for e in e.iter() {
                for g in g.iter() {
                    for n in n.iter() {
                        for m in m.iter() {
                            for a in a.iter() {
                                for x in 1..=26 {
                                    if HashSet::from([*l, *u, *e, *g, *n, *m, *a, x]).len() != 8 {
                                        continue;
                                    }

                                    if l + u +e + g + e + n + m + a + x == 75 {
                                        rl.insert(*l);
                                        ru.insert(*u);
                                        re.insert(*e);
                                        rg.insert(*g);
                                        rn.insert(*n);
                                        rm.insert(*m);
                                        ra.insert(*a);
                                        rx.insert(x);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    (rl, ru, re, rg, rn, rm, ra, rx)
}

fn kniffel(k: Set, n: Set, i: Set, f: Set, e: Set, l: Set) -> (Set, Set, Set, Set, Set, Set) {
    let mut rk = HashSet::new();
    let mut rn = HashSet::new();
    let mut ri = HashSet::new();
    let mut rf = HashSet::new();
    let mut re = HashSet::new();
    let mut rl = HashSet::new();

    for k in k.iter() {
        for n in n.iter() {
            for i in i.iter() {
                for f in f.iter() {
                    for e in e.iter() {
                        for l in l.iter() {
                            if HashSet::from([*k, *n, *i, *f, *e, *l]).len() != 6 {
                                continue;
                            }

                            if k + n + i + f + f + e + l == 79 {
                                rk.insert(*k);
                                rn.insert(*n);
                                ri.insert(*i);
                                rf.insert(*f);
                                re.insert(*e);
                                rl.insert(*l);
                            }
                        }
                    }
                }
            }
        }
    }

    (rk, rn, ri, rf, re, rl)
}

fn wuerfeln(u: Set, e: Set, r: Set, f: Set, l: Set, n: Set) -> (Set, Set, Set, Set, Set, Set, Set) {
    let mut rw = HashSet::new();
    let mut ru = HashSet::new();
    let mut re = HashSet::new();
    let mut rr = HashSet::new();
    let mut rf = HashSet::new();
    let mut rl = HashSet::new();
    let mut rn = HashSet::new();

    for w in 1..=26 {
        for u in u.iter() {
            for e in e.iter() {
                for r in r.iter() {
                    for f in f.iter() {
                        for l in l.iter() {
                            for n in n.iter() {
                                if HashSet::from([w, *u, *e, *r, *f, *l, *n]).len() != 7 {
                                    continue;
                                }

                                if w + u + e + r + f + e + l + n == 80 {
                                    rw.insert(w);
                                    ru.insert(*u);
                                    re.insert(*e);
                                    rr.insert(*r);
                                    rf.insert(*f);
                                    rl.insert(*l);
                                    rn.insert(*n);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    (rw, ru, re, rr, rf, rl, rn)
}

fn risiko(r: Set, i: Set, s: Set, k: Set, o: Set) -> (Set, Set, Set, Set, Set) {
    let mut rr = HashSet::new();
    let mut ri = HashSet::new();
    let mut rs = HashSet::new();
    let mut rk = HashSet::new();
    let mut ro = HashSet::new();

    for r in r.iter() {
        for i in i.iter() {
            for s in s.iter() {
                for k in k.iter() {
                    for o in o.iter() {
                        if HashSet::from([*r, *i, *s, *k, *o]).len() != 5 {
                            continue;
                        }

                        if r + i + s + i + k + o == 82 {
                            rr.insert(*r);
                            ri.insert(*i);
                            rs.insert(*s);
                            rk.insert(*k);
                            ro.insert(*o);
                        }
                    }
                }
            }
        }
    }

    (rr, ri, rs, rk, ro)
}

fn canasta(c: Set, a: Set, n: Set, s: Set, t: Set) -> (Set, Set, Set, Set, Set) {
    let mut rc = HashSet::new();
    let mut ra = HashSet::new();
    let mut rn = HashSet::new();
    let mut rs = HashSet::new();
    let mut rt = HashSet::new();

    for c in c.iter() {
        for a in a.iter() {
            for n in n.iter() {
                for s in s.iter() {
                    for t in t.iter() {
                        if HashSet::from([*c, *a, *n, *s, *t]).len() != 5 {
                            continue;
                        }

                        if c + a + n + a + s + t + a == 87 {
                            rc.insert(*c);
                            ra.insert(*a);
                            rn.insert(*n);
                            rs.insert(*s);
                            rt.insert(*t);
                        }
                    }
                }
            }
        }
    }

    (rc, ra, rn, rs, rt)
}

fn abalone(a: Set, b: Set, l: Set, o: Set, n: Set, e: Set) -> (Set, Set, Set, Set, Set, Set) {
    let mut ra = HashSet::new();
    let mut rb = HashSet::new();
    let mut rl = HashSet::new();
    let mut ro = HashSet::new();
    let mut rn = HashSet::new();
    let mut re = HashSet::new();

    for a in a.iter() {
        for b in b.iter() {
            for l in l.iter() {
                for o in o.iter() {
                    for n in n.iter() {
                        for e in e.iter() {
                            if HashSet::from([*a, *b, *l, *o, *n, *e]).len() != 6 {
                                continue;
                            }

                            if a + b + a + l + o + n + e == 88 {
                                ra.insert(*a);
                                rb.insert(*b);
                                rl.insert(*l);
                                ro.insert(*o);
                                rn.insert(*n);
                                re.insert(*e);
                            }
                        }
                    }
                }
            }
        }
    }

    (ra, rb, rl, ro, rn, re)
}

fn domino(d: Set, o: Set, m: Set, i: Set, n: Set) -> (Set, Set, Set, Set, Set) {
    let mut rd = HashSet::new();
    let mut ro = HashSet::new();
    let mut rm = HashSet::new();
    let mut ri = HashSet::new();
    let mut rn = HashSet::new();

    for d in d.iter() {
        for o in o.iter() {
            for i in i.iter() {
                for m in m.iter() {
                    for n in n.iter() {
                        if HashSet::from([*d, *o, *m, *i, *n]).len() != 5 {
                            continue;
                        }

                        if d + o + m + i + n + o == 89 {
                            rd.insert(*d);
                            ro.insert(*o);
                            rm.insert(*m);
                            ri.insert(*i);
                            rn.insert(*n);
                        }
                    }
                }
            }
        }
    }

    (rd, ro, rm, ri, rn)
}

fn scrabble(s: Set, c: Set, r: Set, a: Set, b: Set, l: Set, e: Set) -> (Set, Set, Set, Set, Set, Set, Set) {
    let mut rs = HashSet::new();
    let mut rc = HashSet::new();
    let mut rr = HashSet::new();
    let mut ra = HashSet::new();
    let mut rb = HashSet::new();
    let mut rl = HashSet::new();
    let mut re = HashSet::new();

    for s in s.iter() {
        for c in c.iter() {
            for r in r.iter() {
                for a in a.iter() {
                    for b in b.iter() {
                        for l in l.iter() {
                            for e in e.iter() {
                                if HashSet::from([*s, *c, *r, *a, *b, *l, *e]).len() != 7 {
                                    continue;
                                }

                                if s + c + r + a + b + b + l + e == 101 {
                                    rs.insert(*s);
                                    rc.insert(*c);
                                    rr.insert(*r);
                                    ra.insert(*a);
                                    rb.insert(*b);
                                    rl.insert(*l);
                                    re.insert(*e);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    (rs, rc, rr, ra, rb, rl, re)
}

fn activity(a: Set, c: Set, t: Set, i: Set, y: Set) -> (Set, Set, Set, Set, Set, Set) {
    let mut ra = HashSet::new();
    let mut rc = HashSet::new();
    let mut rt = HashSet::new();
    let mut ri = HashSet::new();
    let mut rv = HashSet::new();
    let mut ry = HashSet::new();

    for a in a.iter() {
        for c in c.iter() {
            for t in t.iter() {
                for i in i.iter() {
                    for v in 1..=26 {
                        for y in y.iter() {
                            if HashSet::from([*a, *c, *t, *i, v, *y]).len() != 6 {
                                continue;
                            }

                            if a + c + t + i + v + i + t + y == 102 {
                                ra.insert(*a);
                                rc.insert(*c);
                                rt.insert(*t);
                                ri.insert(*i);
                                rv.insert(v);
                                ry.insert(*y);
                            }
                        }
                    }
                }
            }
        }
    }

    (ra, rc, rt, ri, rv, ry)
}

fn ubongo(u: Set, b: Set, o: Set, n: Set, g: Set) -> (Set, Set, Set, Set, Set) {
    let mut ru = HashSet::new();
    let mut rb = HashSet::new();
    let mut ro = HashSet::new();
    let mut rn = HashSet::new();
    let mut rg = HashSet::new();

    for u in u.iter() {
        for b in b.iter() {
            for o in o.iter() {
                for n in n.iter() {
                    for g in g.iter() {
                        if HashSet::from([*u, *b, *o, *n, *g]).len() != 5 {
                            continue;
                        }

                        if u + b + o + n + g + o == 117 {
                            ru.insert(*u);
                            rb.insert(*b);
                            ro.insert(*o);
                            rn.insert(*n);
                            rg.insert(*g);
                        }
                    }
                }
            }
        }
    }

    (ru, rb, ro, rn, rg)
}


fn monopoly(m: Set, o: Set, n: Set, l: Set, y: Set) -> (Set, Set, Set, Set, Set, Set) {
    let mut rm = HashSet::new();
    let mut ro = HashSet::new();
    let mut rn = HashSet::new();
    let mut rp = HashSet::new();
    let mut rl = HashSet::new();
    let mut ry = HashSet::new();

    for m in m.iter() {
        for o in o.iter() {
            for n in n.iter() {
                for p in 1..=26 {
                    for l in l.iter() {
                        for y in y.iter() {
                            if HashSet::from([*m, *o, *n, p, *l, *y]).len() != 6 {
                                continue;
                            }

                            if m + o + n + o + p + o + l + y == 130 {
                                rm.insert(*m);
                                ro.insert(*o);
                                rn.insert(*n);
                                rp.insert(p);
                                rl.insert(*l);
                                ry.insert(*y);
                            }
                        }
                    }
                }
            }
        }
    }

    (rm, ro, rn, rp, rl, ry)
}

fn solution(
    a: Set, b: Set, c: Set, d: Set, e: Set, f: Set, g: Set, h: Set, i: Set, j: Set,
    k: Set, l: Set, m: Set, n: Set, o: Set, p: Set, q: Set, r: Set, s: Set, t: Set,
    u: Set, v: Set, w: Set, x: Set, y: Set, z: Set, combinations: usize
) {
    let timer = std::time::Instant::now();
    let combinations_orig = combinations;
    let combinations = Arc::new(AtomicUsize::new(combinations));
    let mut threads = Vec::new();

    for a in a.clone().into_iter() {
        for b in b.clone().into_iter() {
            if HashSet::from([a, b]).len() != 2 {
                let val = combinations.fetch_sub(1, Ordering::SeqCst);
                if val % 1_000_000 == 0 {
                    let per_sec = (combinations_orig - val) as f32 / timer.elapsed().as_secs_f32();
                    println!("[{:15}] Count {:15} AVG: {:10} REM: {:10}", timer.elapsed().as_secs(), val, per_sec as usize, (val as f32 / per_sec) as usize);
                }

                continue;
            }

            for d in d.clone().into_iter() {
                if HashSet::from([a, b, d]).len() != 3 {
                    let val = combinations.fetch_sub(1, Ordering::SeqCst);
                    if val % 1_000_000 == 0 {
                        let per_sec = (combinations_orig - val) as f32 / timer.elapsed().as_secs_f32();
                        println!("[{:15}] Count {:15} AVG: {:10} REM: {:10}", timer.elapsed().as_secs(), val, per_sec as usize, (val as f32 / per_sec) as usize);
                    }
                    continue;
                }

                let c = c.clone();
                let e = e.clone();
                let f = f.clone();
                let g = g.clone();
                let h = h.clone();
                let i = i.clone();
                let j = j.clone();
                let k = k.clone();
                let l = l.clone();
                let m = m.clone();
                let n = n.clone();
                let o = o.clone();
                let p = p.clone();
                let q = q.clone();
                let r = r.clone();
                let s = s.clone();
                let t = t.clone();
                let u = u.clone();
                let v = v.clone();
                let w = w.clone();
                let x = x.clone();
                let y = y.clone();
                let z = z.clone();
                let combinations = combinations.clone();

                let thread = std::thread::spawn(move || {
                    for c in c.iter() {
                        let mut set = HashSet::from([a, b, *c, d]);

                        if set.len() != 4 {
                            let val = combinations.fetch_sub(1, Ordering::SeqCst);
                            if val % 1_000_000 == 0 {
                                let per_sec = (combinations_orig - val) as f32 / timer.elapsed().as_secs_f32();
                                println!("[{:15}] Count {:15} AVG: {:10} REM: {:10}", timer.elapsed().as_secs(), val, per_sec as usize, (val as f32 / per_sec) as usize);
                            }
                            continue;
                        }

                        for e in e.iter() {
                            if !set.insert(*e) {
                                let val = combinations.fetch_sub(1, Ordering::SeqCst);
                                if val % 1_000_000 == 0 {
                                    let per_sec = (combinations_orig - val) as f32 / timer.elapsed().as_secs_f32();
                                    println!("[{:15}] Count {:15} AVG: {:10} REM: {:10}", timer.elapsed().as_secs(), val, per_sec as usize, (val as f32 / per_sec) as usize);
                                }
                                continue;
                            }

                            for f in f.iter() {
                                if !set.insert(*f) {
                                    let val = combinations.fetch_sub(1, Ordering::SeqCst);
                                    if val % 1_000_000 == 0 {
                                        let per_sec = (combinations_orig - val) as f32 / timer.elapsed().as_secs_f32();
                                        println!("[{:15}] Count {:15} AVG: {:10} REM: {:10}", timer.elapsed().as_secs(), val, per_sec as usize, (val as f32 / per_sec) as usize);
                                    }
                                    continue;
                                }

                                for g in g.iter() {
                                    if !set.insert(*g) {
                                        let val = combinations.fetch_sub(1, Ordering::SeqCst);
                                        if val % 1_000_000 == 0 {
                                            let per_sec = (combinations_orig - val) as f32 / timer.elapsed().as_secs_f32();
                                            println!("[{:15}] Count {:15} AVG: {:10} REM: {:10}", timer.elapsed().as_secs(), val, per_sec as usize, (val as f32 / per_sec) as usize);
                                        }
                                        continue;
                                    }

                                    for h in h.iter() {
                                        if !set.insert(*h) {
                                            let val = combinations.fetch_sub(1, Ordering::SeqCst);
                                            if val % 1_000_000 == 0 {
                                                let per_sec = (combinations_orig - val) as f32 / timer.elapsed().as_secs_f32();
                                                println!("[{:15}] Count {:15} AVG: {:10} REM: {:10}", timer.elapsed().as_secs(), val, per_sec as usize, (val as f32 / per_sec) as usize);
                                            }
                                            continue;
                                        }

                                        for i in i.iter() {
                                            if !set.insert(*i) {
                                                let val = combinations.fetch_sub(1, Ordering::SeqCst);
                                                if val % 1_000_000 == 0 {
                                                    let per_sec = (combinations_orig - val) as f32 / timer.elapsed().as_secs_f32();
                                                    println!("[{:15}] Count {:15} AVG: {:10} REM: {:10}", timer.elapsed().as_secs(), val, per_sec as usize, (val as f32 / per_sec) as usize);
                                                }
                                                continue;
                                            }

                                            for j in j.iter() {
                                                if !set.insert(*j) {
                                                    let val = combinations.fetch_sub(1, Ordering::SeqCst);
                                                    if val % 1_000_000 == 0 {
                                                        let per_sec = (combinations_orig - val) as f32 / timer.elapsed().as_secs_f32();
                                                        println!("[{:15}] Count {:15} AVG: {:10} REM: {:10}", timer.elapsed().as_secs(), val, per_sec as usize, (val as f32 / per_sec) as usize);
                                                    }
                                                    continue;
                                                }

                                                for k in k.iter() {
                                                    if HashSet::from([a, b, *c, d, *e, *f, *g, *h, *i, *j, *k]).len() != 11 {
                                                        let val = combinations.fetch_sub(1, Ordering::SeqCst);
                                                        if val % 1_000_000 == 0 {
                                                            let per_sec = (combinations_orig - val) as f32 / timer.elapsed().as_secs_f32();
                                                            println!("[{:15}] Count {:15} AVG: {:10} REM: {:10}", timer.elapsed().as_secs(), val, per_sec as usize, (val as f32 / per_sec) as usize);
                                                        }
                                                        continue;
                                                    }

                                                    for l in l.iter() {
                                                        if HashSet::from([a, b, *c, d, *e, *f, *g, *h, *i, *j, *k, *l]).len() != 12 {
                                                            let val = combinations.fetch_sub(1, Ordering::SeqCst);
                                                            if val % 1_000_000 == 0 {
                                                                let per_sec = (combinations_orig - val) as f32 / timer.elapsed().as_secs_f32();
                                                                println!("[{:15}] Count {:15} AVG: {:10} REM: {:10}", timer.elapsed().as_secs(), val, per_sec as usize, (val as f32 / per_sec) as usize);
                                                            }
                                                            continue;
                                                        }

                                                        for m in m.iter() {
                                                            if HashSet::from([a, b, *c, d, *e, *f, *g, *h, *i, *j, *k, *l, *m]).len() != 13 {
                                                                let val = combinations.fetch_sub(1, Ordering::SeqCst);
                                                                if val % 1_000_000 == 0 {
                                                                    let per_sec = (combinations_orig - val) as f32 / timer.elapsed().as_secs_f32();
                                                                    println!("[{:15}] Count {:15} AVG: {:10} REM: {:10}", timer.elapsed().as_secs(), val, per_sec as usize, (val as f32 / per_sec) as usize);
                                                                }
                                                                continue;
                                                            }

                                                            for n in n.iter() {
                                                                if HashSet::from([a, b, *c, d, *e, *f, *g, *h, *i, *j, *k, *l, *m, *n]).len() != 14 {
                                                                    let val = combinations.fetch_sub(1, Ordering::SeqCst);
                                                                    if val % 1_000_000 == 0 {
                                                                        let per_sec = (combinations_orig - val) as f32 / timer.elapsed().as_secs_f32();
                                                                        println!("[{:15}] Count {:15} AVG: {:10} REM: {:10}", timer.elapsed().as_secs(), val, per_sec as usize, (val as f32 / per_sec) as usize);
                                                                    }
                                                                    continue;
                                                                }

                                                                for o in o.iter() {
                                                                    if HashSet::from([a, b, *c, d, *e, *f, *g, *h, *i, *j, *k, *l, *m, *n, *o]).len() != 15 {
                                                                        let val = combinations.fetch_sub(1, Ordering::SeqCst);
                                                                        if val % 1_000_000 == 0 {
                                                                            let per_sec = (combinations_orig - val) as f32 / timer.elapsed().as_secs_f32();
                                                                            println!("[{:15}] Count {:15} AVG: {:10} REM: {:10}", timer.elapsed().as_secs(), val, per_sec as usize, (val as f32 / per_sec) as usize);
                                                                        }
                                                                        continue;
                                                                    }

                                                                    for p in p.iter() {
                                                                        if HashSet::from([a, b, *c, d, *e, *f, *g, *h, *i, *j, *k, *l, *m, *n, *o, *p]).len() != 16 {
                                                                            let val = combinations.fetch_sub(1, Ordering::SeqCst);
                                                                            if val % 1_000_000 == 0 {
                                                                                let per_sec = (combinations_orig - val) as f32 / timer.elapsed().as_secs_f32();
                                                                                println!("[{:15}] Count {:15} AVG: {:10} REM: {:10}", timer.elapsed().as_secs(), val, per_sec as usize, (val as f32 / per_sec) as usize);
                                                                            }
                                                                            continue;
                                                                        }

                                                                        for q in q.iter() {
                                                                            if HashSet::from([a, b, *c, d, *e, *f, *g, *h, *i, *j, *k, *l, *m, *n, *o, *p, *q]).len() != 17 {
                                                                                let val = combinations.fetch_sub(1, Ordering::SeqCst);
                                                                                if val % 1_000_000 == 0 {
                                                                                    let per_sec = (combinations_orig - val) as f32 / timer.elapsed().as_secs_f32();
                                                                                    println!("[{:15}] Count {:15} AVG: {:10} REM: {:10}", timer.elapsed().as_secs(), val, per_sec as usize, (val as f32 / per_sec) as usize);
                                                                                }
                                                                                continue;
                                                                            }

                                                                            for r in r.iter() {
                                                                                if HashSet::from([a, b, *c, d, *e, *f, *g, *h, *i, *j, *k, *l, *m, *n, *o, *p, *q, *r]).len() != 18 {
                                                                                    let val = combinations.fetch_sub(1, Ordering::SeqCst);
                                                                                    if val % 1_000_000 == 0 {
                                                                                        let per_sec = (combinations_orig - val) as f32 / timer.elapsed().as_secs_f32();
                                                                                        println!("[{:15}] Count {:15} AVG: {:10} REM: {:10}", timer.elapsed().as_secs(), val, per_sec as usize, (val as f32 / per_sec) as usize);
                                                                                    }
                                                                                    continue;
                                                                                }

                                                                                for s in s.iter() {
                                                                                    if HashSet::from([a, b, *c, d, *e, *f, *g, *h, *i, *j, *k, *l, *m, *n, *o, *p, *q, *r, *s]).len() != 19 {
                                                                                        let val = combinations.fetch_sub(1, Ordering::SeqCst);
                                                                                        if val % 1_000_000 == 0 {
                                                                                            let per_sec = (combinations_orig - val) as f32 / timer.elapsed().as_secs_f32();
                                                                                            println!("[{:15}] Count {:15} AVG: {:10} REM: {:10}", timer.elapsed().as_secs(), val, per_sec as usize, (val as f32 / per_sec) as usize);
                                                                                        }
                                                                                        continue;
                                                                                    }

                                                                                    for t in t.iter() {
                                                                                        if HashSet::from([a, b, *c, d, *e, *f, *g, *h, *i, *j, *k, *l, *m, *n, *o, *p, *q, *r, *s, *t]).len() != 20 {
                                                                                            let val = combinations.fetch_sub(1, Ordering::SeqCst);
                                                                                            if val % 1_000_000 == 0 {
                                                                                                let per_sec = (combinations_orig - val) as f32 / timer.elapsed().as_secs_f32();
                                                                                                println!("[{:15}] Count {:15} AVG: {:10} REM: {:10}", timer.elapsed().as_secs(), val, per_sec as usize, (val as f32 / per_sec) as usize);
                                                                                            }
                                                                                            continue;
                                                                                        }

                                                                                        for u in u.iter() {
                                                                                            if HashSet::from([a, b, *c, d, *e, *f, *g, *h, *i, *j, *k, *l, *m, *n, *o, *p, *q, *r, *s, *t, *u]).len() != 21 {
                                                                                                let val = combinations.fetch_sub(1, Ordering::SeqCst);
                                                                                                if val % 1_000_000 == 0 {
                                                                                                    let per_sec = (combinations_orig - val) as f32 / timer.elapsed().as_secs_f32();
                                                                                                    println!("[{:15}] Count {:15} AVG: {:10} REM: {:10}", timer.elapsed().as_secs(), val, per_sec as usize, (val as f32 / per_sec) as usize);
                                                                                                }
                                                                                                continue;
                                                                                            }

                                                                                            for v in v.iter() {
                                                                                                if HashSet::from([a, b, *c, d, *e, *f, *g, *h, *i, *j, *k, *l, *m, *n, *o, *p, *q, *r, *s, *t, *u, *v]).len() != 22 {
                                                                                                    let val = combinations.fetch_sub(1, Ordering::SeqCst);
                                                                                                    if val % 1_000_000 == 0 {
                                                                                                        let per_sec = (combinations_orig - val) as f32 / timer.elapsed().as_secs_f32();
                                                                                                        println!("[{:15}] Count {:15} AVG: {:10} REM: {:10}", timer.elapsed().as_secs(), val, per_sec as usize, (val as f32 / per_sec) as usize);
                                                                                                    }
                                                                                                    continue;
                                                                                                }

                                                                                                for w in w.iter() {
                                                                                                    if HashSet::from([a, b, *c, d, *e, *f, *g, *h, *i, *j, *k, *l, *m, *n, *o, *p, *q, *r, *s, *t, *u, *v, *w]).len() != 23 {
                                                                                                        let val = combinations.fetch_sub(1, Ordering::SeqCst);
                                                                                                        if val % 1_000_000 == 0 {
                                                                                                            let per_sec = (combinations_orig - val) as f32 / timer.elapsed().as_secs_f32();
                                                                                                            println!("[{:15}] Count {:15} AVG: {:10} REM: {:10}", timer.elapsed().as_secs(), val, per_sec as usize, (val as f32 / per_sec) as usize);
                                                                                                        }
                                                                                                        continue;
                                                                                                    }

                                                                                                    for x in x.iter() {
                                                                                                        if HashSet::from([a, b, *c, d, *e, *f, *g, *h, *i, *j, *k, *l, *m, *n, *o, *p, *q, *r, *s, *t, *u, *v, *w, *x]).len() != 24 {
                                                                                                            let val = combinations.fetch_sub(1, Ordering::SeqCst);
                                                                                                            if val % 1_000_000 == 0 {
                                                                                                                let per_sec = (combinations_orig - val) as f32 / timer.elapsed().as_secs_f32();
                                                                                                                println!("[{:15}] Count {:15} AVG: {:10} REM: {:10}", timer.elapsed().as_secs(), val, per_sec as usize, (val as f32 / per_sec) as usize);
                                                                                                            }
                                                                                                            continue;
                                                                                                        }

                                                                                                        for y in y.iter() {
                                                                                                            if HashSet::from([a, b, *c, d, *e, *f, *g, *h, *i, *j, *k, *l, *m, *n, *o, *p, *q, *r, *s, *t, *u, *v, *w, *x, *y]).len() != 25 {
                                                                                                                let val = combinations.fetch_sub(1, Ordering::SeqCst);
                                                                                                                if val % 1_000_000 == 0 {
                                                                                                                    let per_sec = (combinations_orig - val) as f32 / timer.elapsed().as_secs_f32();
                                                                                                                    println!("[{:15}] Count {:15} AVG: {:10} REM: {:10}", timer.elapsed().as_secs(), val, per_sec as usize, (val as f32 / per_sec) as usize);
                                                                                                                }
                                                                                                                continue;
                                                                                                            }

                                                                                                            for z in z.iter() {
                                                                                                                if HashSet::from([a, b, *c, d, *e, *f, *g, *h, *i, *j, *k, *l, *m, *n, *o, *p, *q, *r, *s, *t, *u, *v, *w, *x, *y, *z]).len() != 26 {
                                                                                                                    let val = combinations.fetch_sub(1, Ordering::SeqCst);
                                                                                                                    if val % 1_000_000 == 0 {
                                                                                                                        let per_sec = (combinations_orig - val) as f32 / timer.elapsed().as_secs_f32();
                                                                                                                        println!("[{:15}] Count {:15} AVG: {:10} REM: {:10}", timer.elapsed().as_secs(), val, per_sec as usize, (val as f32 / per_sec) as usize);
                                                                                                                    }
                                                                                                                    continue;
                                                                                                                }

                                                                                                                let val = combinations.fetch_sub(1, Ordering::SeqCst);
                                                                                                                if val % 1_000_000 == 0 {
                                                                                                                    let per_sec = (combinations_orig - val) as f32 / timer.elapsed().as_secs_f32();
                                                                                                                    println!("[{:15}] Count {:15} AVG: {:10} REM: {:10}", timer.elapsed().as_secs(), val, per_sec as usize, (val as f32 / per_sec) as usize);
                                                                                                                }

                                                                                                                if d + a + m + e != 11 {
                                                                                                                    continue;
                                                                                                                }

                                                                                                                if m + a + u + m + a + u != 14 {
                                                                                                                    continue;
                                                                                                                }

                                                                                                                if h + a + l + m + a != 19 {
                                                                                                                    continue;
                                                                                                                }

                                                                                                                if m + u + e + h + l + e != 25 {
                                                                                                                    continue;
                                                                                                                }

                                                                                                                if m + a + l + e + f + i + z != 50 {
                                                                                                                    continue;
                                                                                                                }

                                                                                                                if m + e + m + o + r + y != 51 {
                                                                                                                    continue;
                                                                                                                }

                                                                                                                if t + a + b + u != 52 {
                                                                                                                    continue;
                                                                                                                }

                                                                                                                if q + u + i + z != 54 {
                                                                                                                    continue;
                                                                                                                }

                                                                                                                if m + i + k + a + d + o != 55 {
                                                                                                                    continue;
                                                                                                                }

                                                                                                                if u + n + o != 55 {
                                                                                                                    continue;
                                                                                                                }

                                                                                                                if c + l + u + e + d + o != 57 {
                                                                                                                    continue;
                                                                                                                }

                                                                                                                if b + r + i + d + g + e != 61 {
                                                                                                                    continue;
                                                                                                                }

                                                                                                                if j + e + n + g + a != 65 {
                                                                                                                    continue;
                                                                                                                }

                                                                                                                if s + c + h + a + c + h != 65 {
                                                                                                                    continue;
                                                                                                                }

                                                                                                                if l + u + e + g + e + n + m + a + x != 75 {
                                                                                                                    continue;
                                                                                                                }

                                                                                                                if k + n + i + f + f + e + l != 79 {
                                                                                                                    continue;
                                                                                                                }

                                                                                                                if w + u + e + r + f + e + l + n != 80 {
                                                                                                                    continue;
                                                                                                                }

                                                                                                                if r + i + s + i + k + o != 82 {
                                                                                                                    continue;
                                                                                                                }

                                                                                                                if c + a + n + a + s + t + a != 87 {
                                                                                                                    continue;
                                                                                                                }

                                                                                                                if a + b + a + l + o + n + e != 88 {
                                                                                                                    continue;
                                                                                                                }

                                                                                                                if d + o + m + i + n + o != 89 {
                                                                                                                    continue;
                                                                                                                }

                                                                                                                if s + c + r + a + b + b + l + e != 101 {
                                                                                                                    continue;
                                                                                                                }

                                                                                                                if a + c + t + i + v + i + t + y != 102 {
                                                                                                                    continue;
                                                                                                                }

                                                                                                                if u + b + o + n + g + o != 117 {
                                                                                                                    continue;
                                                                                                                }

                                                                                                                if m + o + n + o + p + o + l + y != 130 {
                                                                                                                    continue;
                                                                                                                }

                                                                                                                let res = format!("A: {}, B: {}, C: {}, D: {}, E: {}, F: {}, G: {}, H: {}, I: {}, J: {}, K: {}, L: {}, M: {}, N: {}, O: {}, P: {}, Q: {}, R: {}, S: {}, T: {}, U: {}, V: {}, W: {}, X: {}, Y: {}, Z: {}\n", a, b, *c, d, *e, *f, *g, *h, *i, *j, *k, *l, *m, *n, *o, *p, *q, *r, *s, *t, *u, *v, w, x, y, z);
                                                                                                                let mut file = std::fs::File::open("./results").unwrap();
                                                                                                                file.write_all(res.as_bytes()).unwrap();
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                });

                threads.push(thread);
            }
        }
    }

    dbg!(threads.len());

    for thread in threads {
        thread.join().unwrap();
    }
}
