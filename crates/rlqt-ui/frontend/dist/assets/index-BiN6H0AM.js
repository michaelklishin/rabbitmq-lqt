(async ()=>{
    (function() {
        const i = document.createElement("link").relList;
        if (i && i.supports && i.supports("modulepreload")) return;
        for (const c of document.querySelectorAll('link[rel="modulepreload"]'))r(c);
        new MutationObserver((c)=>{
            for (const d of c)if (d.type === "childList") for (const m of d.addedNodes)m.tagName === "LINK" && m.rel === "modulepreload" && r(m);
        }).observe(document, {
            childList: !0,
            subtree: !0
        });
        function u(c) {
            const d = {};
            return c.integrity && (d.integrity = c.integrity), c.referrerPolicy && (d.referrerPolicy = c.referrerPolicy), c.crossOrigin === "use-credentials" ? d.credentials = "include" : c.crossOrigin === "anonymous" ? d.credentials = "omit" : d.credentials = "same-origin", d;
        }
        function r(c) {
            if (c.ep) return;
            c.ep = !0;
            const d = u(c);
            fetch(c.href, d);
        }
    })();
    var Fr = {
        exports: {}
    }, Xa = {};
    var Oh;
    function ay() {
        if (Oh) return Xa;
        Oh = 1;
        var l = Symbol.for("react.transitional.element"), i = Symbol.for("react.fragment");
        function u(r, c, d) {
            var m = null;
            if (d !== void 0 && (m = "" + d), c.key !== void 0 && (m = "" + c.key), "key" in c) {
                d = {};
                for(var v in c)v !== "key" && (d[v] = c[v]);
            } else d = c;
            return c = d.ref, {
                $$typeof: l,
                type: r,
                key: m,
                ref: c !== void 0 ? c : null,
                props: d
            };
        }
        return Xa.Fragment = i, Xa.jsx = u, Xa.jsxs = u, Xa;
    }
    var Th;
    function iy() {
        return Th || (Th = 1, Fr.exports = ay()), Fr.exports;
    }
    var h = iy(), Zr = {
        exports: {}
    }, ee = {};
    var Nh;
    function uy() {
        if (Nh) return ee;
        Nh = 1;
        var l = Symbol.for("react.transitional.element"), i = Symbol.for("react.portal"), u = Symbol.for("react.fragment"), r = Symbol.for("react.strict_mode"), c = Symbol.for("react.profiler"), d = Symbol.for("react.consumer"), m = Symbol.for("react.context"), v = Symbol.for("react.forward_ref"), y = Symbol.for("react.suspense"), p = Symbol.for("react.memo"), _ = Symbol.for("react.lazy"), M = Symbol.for("react.activity"), j = Symbol.iterator;
        function T(x) {
            return x === null || typeof x != "object" ? null : (x = j && x[j] || x["@@iterator"], typeof x == "function" ? x : null);
        }
        var U = {
            isMounted: function() {
                return !1;
            },
            enqueueForceUpdate: function() {},
            enqueueReplaceState: function() {},
            enqueueSetState: function() {}
        }, q = Object.assign, B = {};
        function G(x, H, L) {
            this.props = x, this.context = H, this.refs = B, this.updater = L || U;
        }
        G.prototype.isReactComponent = {}, G.prototype.setState = function(x, H) {
            if (typeof x != "object" && typeof x != "function" && x != null) throw Error("takes an object of state variables to update or a function which returns an object of state variables.");
            this.updater.enqueueSetState(this, x, H, "setState");
        }, G.prototype.forceUpdate = function(x) {
            this.updater.enqueueForceUpdate(this, x, "forceUpdate");
        };
        function ae() {}
        ae.prototype = G.prototype;
        function Z(x, H, L) {
            this.props = x, this.context = H, this.refs = B, this.updater = L || U;
        }
        var oe = Z.prototype = new ae;
        oe.constructor = Z, q(oe, G.prototype), oe.isPureReactComponent = !0;
        var he = Array.isArray;
        function F() {}
        var $ = {
            H: null,
            A: null,
            T: null,
            S: null
        }, I = Object.prototype.hasOwnProperty;
        function Me(x, H, L) {
            var Y = L.ref;
            return {
                $$typeof: l,
                type: x,
                key: H,
                ref: Y !== void 0 ? Y : null,
                props: L
            };
        }
        function Pe(x, H) {
            return Me(x.type, H, x.props);
        }
        function Be(x) {
            return typeof x == "object" && x !== null && x.$$typeof === l;
        }
        function tt(x) {
            var H = {
                "=": "=0",
                ":": "=2"
            };
            return "$" + x.replace(/[=:]/g, function(L) {
                return H[L];
            });
        }
        var Ln = /\/+/g;
        function Qt(x, H) {
            return typeof x == "object" && x !== null && x.key != null ? tt("" + x.key) : H.toString(36);
        }
        function At(x) {
            switch(x.status){
                case "fulfilled":
                    return x.value;
                case "rejected":
                    throw x.reason;
                default:
                    switch(typeof x.status == "string" ? x.then(F, F) : (x.status = "pending", x.then(function(H) {
                        x.status === "pending" && (x.status = "fulfilled", x.value = H);
                    }, function(H) {
                        x.status === "pending" && (x.status = "rejected", x.reason = H);
                    })), x.status){
                        case "fulfilled":
                            return x.value;
                        case "rejected":
                            throw x.reason;
                    }
            }
            throw x;
        }
        function A(x, H, L, Y, te) {
            var ue = typeof x;
            (ue === "undefined" || ue === "boolean") && (x = null);
            var ve = !1;
            if (x === null) ve = !0;
            else switch(ue){
                case "bigint":
                case "string":
                case "number":
                    ve = !0;
                    break;
                case "object":
                    switch(x.$$typeof){
                        case l:
                        case i:
                            ve = !0;
                            break;
                        case _:
                            return ve = x._init, A(ve(x._payload), H, L, Y, te);
                    }
            }
            if (ve) return te = te(x), ve = Y === "" ? "." + Qt(x, 0) : Y, he(te) ? (L = "", ve != null && (L = ve.replace(Ln, "$&/") + "/"), A(te, H, L, "", function(Wl) {
                return Wl;
            })) : te != null && (Be(te) && (te = Pe(te, L + (te.key == null || x && x.key === te.key ? "" : ("" + te.key).replace(Ln, "$&/") + "/") + ve)), H.push(te)), 1;
            ve = 0;
            var Ie = Y === "" ? "." : Y + ":";
            if (he(x)) for(var ze = 0; ze < x.length; ze++)Y = x[ze], ue = Ie + Qt(Y, ze), ve += A(Y, H, L, ue, te);
            else if (ze = T(x), typeof ze == "function") for(x = ze.call(x), ze = 0; !(Y = x.next()).done;)Y = Y.value, ue = Ie + Qt(Y, ze++), ve += A(Y, H, L, ue, te);
            else if (ue === "object") {
                if (typeof x.then == "function") return A(At(x), H, L, Y, te);
                throw H = String(x), Error("Objects are not valid as a React child (found: " + (H === "[object Object]" ? "object with keys {" + Object.keys(x).join(", ") + "}" : H) + "). If you meant to render a collection of children, use an array instead.");
            }
            return ve;
        }
        function V(x, H, L) {
            if (x == null) return x;
            var Y = [], te = 0;
            return A(x, Y, "", "", function(ue) {
                return H.call(L, ue, te++);
            }), Y;
        }
        function k(x) {
            if (x._status === -1) {
                var H = x._result;
                H = H(), H.then(function(L) {
                    (x._status === 0 || x._status === -1) && (x._status = 1, x._result = L);
                }, function(L) {
                    (x._status === 0 || x._status === -1) && (x._status = 2, x._result = L);
                }), x._status === -1 && (x._status = 0, x._result = H);
            }
            if (x._status === 1) return x._result.default;
            throw x._result;
        }
        var xe = typeof reportError == "function" ? reportError : function(x) {
            if (typeof window == "object" && typeof window.ErrorEvent == "function") {
                var H = new window.ErrorEvent("error", {
                    bubbles: !0,
                    cancelable: !0,
                    message: typeof x == "object" && x !== null && typeof x.message == "string" ? String(x.message) : String(x),
                    error: x
                });
                if (!window.dispatchEvent(H)) return;
            } else if (typeof process == "object" && typeof process.emit == "function") {
                process.emit("uncaughtException", x);
                return;
            }
            console.error(x);
        }, we = {
            map: V,
            forEach: function(x, H, L) {
                V(x, function() {
                    H.apply(this, arguments);
                }, L);
            },
            count: function(x) {
                var H = 0;
                return V(x, function() {
                    H++;
                }), H;
            },
            toArray: function(x) {
                return V(x, function(H) {
                    return H;
                }) || [];
            },
            only: function(x) {
                if (!Be(x)) throw Error("React.Children.only expected to receive a single React element child.");
                return x;
            }
        };
        return ee.Activity = M, ee.Children = we, ee.Component = G, ee.Fragment = u, ee.Profiler = c, ee.PureComponent = Z, ee.StrictMode = r, ee.Suspense = y, ee.__CLIENT_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE = $, ee.__COMPILER_RUNTIME = {
            __proto__: null,
            c: function(x) {
                return $.H.useMemoCache(x);
            }
        }, ee.cache = function(x) {
            return function() {
                return x.apply(null, arguments);
            };
        }, ee.cacheSignal = function() {
            return null;
        }, ee.cloneElement = function(x, H, L) {
            if (x == null) throw Error("The argument must be a React element, but you passed " + x + ".");
            var Y = q({}, x.props), te = x.key;
            if (H != null) for(ue in H.key !== void 0 && (te = "" + H.key), H)!I.call(H, ue) || ue === "key" || ue === "__self" || ue === "__source" || ue === "ref" && H.ref === void 0 || (Y[ue] = H[ue]);
            var ue = arguments.length - 2;
            if (ue === 1) Y.children = L;
            else if (1 < ue) {
                for(var ve = Array(ue), Ie = 0; Ie < ue; Ie++)ve[Ie] = arguments[Ie + 2];
                Y.children = ve;
            }
            return Me(x.type, te, Y);
        }, ee.createContext = function(x) {
            return x = {
                $$typeof: m,
                _currentValue: x,
                _currentValue2: x,
                _threadCount: 0,
                Provider: null,
                Consumer: null
            }, x.Provider = x, x.Consumer = {
                $$typeof: d,
                _context: x
            }, x;
        }, ee.createElement = function(x, H, L) {
            var Y, te = {}, ue = null;
            if (H != null) for(Y in H.key !== void 0 && (ue = "" + H.key), H)I.call(H, Y) && Y !== "key" && Y !== "__self" && Y !== "__source" && (te[Y] = H[Y]);
            var ve = arguments.length - 2;
            if (ve === 1) te.children = L;
            else if (1 < ve) {
                for(var Ie = Array(ve), ze = 0; ze < ve; ze++)Ie[ze] = arguments[ze + 2];
                te.children = Ie;
            }
            if (x && x.defaultProps) for(Y in ve = x.defaultProps, ve)te[Y] === void 0 && (te[Y] = ve[Y]);
            return Me(x, ue, te);
        }, ee.createRef = function() {
            return {
                current: null
            };
        }, ee.forwardRef = function(x) {
            return {
                $$typeof: v,
                render: x
            };
        }, ee.isValidElement = Be, ee.lazy = function(x) {
            return {
                $$typeof: _,
                _payload: {
                    _status: -1,
                    _result: x
                },
                _init: k
            };
        }, ee.memo = function(x, H) {
            return {
                $$typeof: p,
                type: x,
                compare: H === void 0 ? null : H
            };
        }, ee.startTransition = function(x) {
            var H = $.T, L = {};
            $.T = L;
            try {
                var Y = x(), te = $.S;
                te !== null && te(L, Y), typeof Y == "object" && Y !== null && typeof Y.then == "function" && Y.then(F, xe);
            } catch (ue) {
                xe(ue);
            } finally{
                H !== null && L.types !== null && (H.types = L.types), $.T = H;
            }
        }, ee.unstable_useCacheRefresh = function() {
            return $.H.useCacheRefresh();
        }, ee.use = function(x) {
            return $.H.use(x);
        }, ee.useActionState = function(x, H, L) {
            return $.H.useActionState(x, H, L);
        }, ee.useCallback = function(x, H) {
            return $.H.useCallback(x, H);
        }, ee.useContext = function(x) {
            return $.H.useContext(x);
        }, ee.useDebugValue = function() {}, ee.useDeferredValue = function(x, H) {
            return $.H.useDeferredValue(x, H);
        }, ee.useEffect = function(x, H) {
            return $.H.useEffect(x, H);
        }, ee.useEffectEvent = function(x) {
            return $.H.useEffectEvent(x);
        }, ee.useId = function() {
            return $.H.useId();
        }, ee.useImperativeHandle = function(x, H, L) {
            return $.H.useImperativeHandle(x, H, L);
        }, ee.useInsertionEffect = function(x, H) {
            return $.H.useInsertionEffect(x, H);
        }, ee.useLayoutEffect = function(x, H) {
            return $.H.useLayoutEffect(x, H);
        }, ee.useMemo = function(x, H) {
            return $.H.useMemo(x, H);
        }, ee.useOptimistic = function(x, H) {
            return $.H.useOptimistic(x, H);
        }, ee.useReducer = function(x, H, L) {
            return $.H.useReducer(x, H, L);
        }, ee.useRef = function(x) {
            return $.H.useRef(x);
        }, ee.useState = function(x) {
            return $.H.useState(x);
        }, ee.useSyncExternalStore = function(x, H, L) {
            return $.H.useSyncExternalStore(x, H, L);
        }, ee.useTransition = function() {
            return $.H.useTransition();
        }, ee.version = "19.2.1", ee;
    }
    var Ah;
    function So() {
        return Ah || (Ah = 1, Zr.exports = uy()), Zr.exports;
    }
    var ie = So(), Kr = {
        exports: {}
    }, Fa = {}, $r = {
        exports: {}
    }, Jr = {};
    var zh;
    function sy() {
        return zh || (zh = 1, (function(l) {
            function i(A, V) {
                var k = A.length;
                A.push(V);
                e: for(; 0 < k;){
                    var xe = k - 1 >>> 1, we = A[xe];
                    if (0 < c(we, V)) A[xe] = V, A[k] = we, k = xe;
                    else break e;
                }
            }
            function u(A) {
                return A.length === 0 ? null : A[0];
            }
            function r(A) {
                if (A.length === 0) return null;
                var V = A[0], k = A.pop();
                if (k !== V) {
                    A[0] = k;
                    e: for(var xe = 0, we = A.length, x = we >>> 1; xe < x;){
                        var H = 2 * (xe + 1) - 1, L = A[H], Y = H + 1, te = A[Y];
                        if (0 > c(L, k)) Y < we && 0 > c(te, L) ? (A[xe] = te, A[Y] = k, xe = Y) : (A[xe] = L, A[H] = k, xe = H);
                        else if (Y < we && 0 > c(te, k)) A[xe] = te, A[Y] = k, xe = Y;
                        else break e;
                    }
                }
                return V;
            }
            function c(A, V) {
                var k = A.sortIndex - V.sortIndex;
                return k !== 0 ? k : A.id - V.id;
            }
            if (l.unstable_now = void 0, typeof performance == "object" && typeof performance.now == "function") {
                var d = performance;
                l.unstable_now = function() {
                    return d.now();
                };
            } else {
                var m = Date, v = m.now();
                l.unstable_now = function() {
                    return m.now() - v;
                };
            }
            var y = [], p = [], _ = 1, M = null, j = 3, T = !1, U = !1, q = !1, B = !1, G = typeof setTimeout == "function" ? setTimeout : null, ae = typeof clearTimeout == "function" ? clearTimeout : null, Z = typeof setImmediate < "u" ? setImmediate : null;
            function oe(A) {
                for(var V = u(p); V !== null;){
                    if (V.callback === null) r(p);
                    else if (V.startTime <= A) r(p), V.sortIndex = V.expirationTime, i(y, V);
                    else break;
                    V = u(p);
                }
            }
            function he(A) {
                if (q = !1, oe(A), !U) if (u(y) !== null) U = !0, F || (F = !0, tt());
                else {
                    var V = u(p);
                    V !== null && At(he, V.startTime - A);
                }
            }
            var F = !1, $ = -1, I = 5, Me = -1;
            function Pe() {
                return B ? !0 : !(l.unstable_now() - Me < I);
            }
            function Be() {
                if (B = !1, F) {
                    var A = l.unstable_now();
                    Me = A;
                    var V = !0;
                    try {
                        e: {
                            U = !1, q && (q = !1, ae($), $ = -1), T = !0;
                            var k = j;
                            try {
                                t: {
                                    for(oe(A), M = u(y); M !== null && !(M.expirationTime > A && Pe());){
                                        var xe = M.callback;
                                        if (typeof xe == "function") {
                                            M.callback = null, j = M.priorityLevel;
                                            var we = xe(M.expirationTime <= A);
                                            if (A = l.unstable_now(), typeof we == "function") {
                                                M.callback = we, oe(A), V = !0;
                                                break t;
                                            }
                                            M === u(y) && r(y), oe(A);
                                        } else r(y);
                                        M = u(y);
                                    }
                                    if (M !== null) V = !0;
                                    else {
                                        var x = u(p);
                                        x !== null && At(he, x.startTime - A), V = !1;
                                    }
                                }
                                break e;
                            } finally{
                                M = null, j = k, T = !1;
                            }
                            V = void 0;
                        }
                    } finally{
                        V ? tt() : F = !1;
                    }
                }
            }
            var tt;
            if (typeof Z == "function") tt = function() {
                Z(Be);
            };
            else if (typeof MessageChannel < "u") {
                var Ln = new MessageChannel, Qt = Ln.port2;
                Ln.port1.onmessage = Be, tt = function() {
                    Qt.postMessage(null);
                };
            } else tt = function() {
                G(Be, 0);
            };
            function At(A, V) {
                $ = G(function() {
                    A(l.unstable_now());
                }, V);
            }
            l.unstable_IdlePriority = 5, l.unstable_ImmediatePriority = 1, l.unstable_LowPriority = 4, l.unstable_NormalPriority = 3, l.unstable_Profiling = null, l.unstable_UserBlockingPriority = 2, l.unstable_cancelCallback = function(A) {
                A.callback = null;
            }, l.unstable_forceFrameRate = function(A) {
                0 > A || 125 < A ? console.error("forceFrameRate takes a positive int between 0 and 125, forcing frame rates higher than 125 fps is not supported") : I = 0 < A ? Math.floor(1e3 / A) : 5;
            }, l.unstable_getCurrentPriorityLevel = function() {
                return j;
            }, l.unstable_next = function(A) {
                switch(j){
                    case 1:
                    case 2:
                    case 3:
                        var V = 3;
                        break;
                    default:
                        V = j;
                }
                var k = j;
                j = V;
                try {
                    return A();
                } finally{
                    j = k;
                }
            }, l.unstable_requestPaint = function() {
                B = !0;
            }, l.unstable_runWithPriority = function(A, V) {
                switch(A){
                    case 1:
                    case 2:
                    case 3:
                    case 4:
                    case 5:
                        break;
                    default:
                        A = 3;
                }
                var k = j;
                j = A;
                try {
                    return V();
                } finally{
                    j = k;
                }
            }, l.unstable_scheduleCallback = function(A, V, k) {
                var xe = l.unstable_now();
                switch(typeof k == "object" && k !== null ? (k = k.delay, k = typeof k == "number" && 0 < k ? xe + k : xe) : k = xe, A){
                    case 1:
                        var we = -1;
                        break;
                    case 2:
                        we = 250;
                        break;
                    case 5:
                        we = 1073741823;
                        break;
                    case 4:
                        we = 1e4;
                        break;
                    default:
                        we = 5e3;
                }
                return we = k + we, A = {
                    id: _++,
                    callback: V,
                    priorityLevel: A,
                    startTime: k,
                    expirationTime: we,
                    sortIndex: -1
                }, k > xe ? (A.sortIndex = k, i(p, A), u(y) === null && A === u(p) && (q ? (ae($), $ = -1) : q = !0, At(he, k - xe))) : (A.sortIndex = we, i(y, A), U || T || (U = !0, F || (F = !0, tt()))), A;
            }, l.unstable_shouldYield = Pe, l.unstable_wrapCallback = function(A) {
                var V = j;
                return function() {
                    var k = j;
                    j = V;
                    try {
                        return A.apply(this, arguments);
                    } finally{
                        j = k;
                    }
                };
            };
        })(Jr)), Jr;
    }
    var Dh;
    function ry() {
        return Dh || (Dh = 1, $r.exports = sy()), $r.exports;
    }
    var kr = {
        exports: {}
    }, ke = {};
    var Hh;
    function oy() {
        if (Hh) return ke;
        Hh = 1;
        var l = So();
        function i(y) {
            var p = "https://react.dev/errors/" + y;
            if (1 < arguments.length) {
                p += "?args[]=" + encodeURIComponent(arguments[1]);
                for(var _ = 2; _ < arguments.length; _++)p += "&args[]=" + encodeURIComponent(arguments[_]);
            }
            return "Minified React error #" + y + "; visit " + p + " for the full message or use the non-minified dev environment for full errors and additional helpful warnings.";
        }
        function u() {}
        var r = {
            d: {
                f: u,
                r: function() {
                    throw Error(i(522));
                },
                D: u,
                C: u,
                L: u,
                m: u,
                X: u,
                S: u,
                M: u
            },
            p: 0,
            findDOMNode: null
        }, c = Symbol.for("react.portal");
        function d(y, p, _) {
            var M = 3 < arguments.length && arguments[3] !== void 0 ? arguments[3] : null;
            return {
                $$typeof: c,
                key: M == null ? null : "" + M,
                children: y,
                containerInfo: p,
                implementation: _
            };
        }
        var m = l.__CLIENT_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE;
        function v(y, p) {
            if (y === "font") return "";
            if (typeof p == "string") return p === "use-credentials" ? p : "";
        }
        return ke.__DOM_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE = r, ke.createPortal = function(y, p) {
            var _ = 2 < arguments.length && arguments[2] !== void 0 ? arguments[2] : null;
            if (!p || p.nodeType !== 1 && p.nodeType !== 9 && p.nodeType !== 11) throw Error(i(299));
            return d(y, p, null, _);
        }, ke.flushSync = function(y) {
            var p = m.T, _ = r.p;
            try {
                if (m.T = null, r.p = 2, y) return y();
            } finally{
                m.T = p, r.p = _, r.d.f();
            }
        }, ke.preconnect = function(y, p) {
            typeof y == "string" && (p ? (p = p.crossOrigin, p = typeof p == "string" ? p === "use-credentials" ? p : "" : void 0) : p = null, r.d.C(y, p));
        }, ke.prefetchDNS = function(y) {
            typeof y == "string" && r.d.D(y);
        }, ke.preinit = function(y, p) {
            if (typeof y == "string" && p && typeof p.as == "string") {
                var _ = p.as, M = v(_, p.crossOrigin), j = typeof p.integrity == "string" ? p.integrity : void 0, T = typeof p.fetchPriority == "string" ? p.fetchPriority : void 0;
                _ === "style" ? r.d.S(y, typeof p.precedence == "string" ? p.precedence : void 0, {
                    crossOrigin: M,
                    integrity: j,
                    fetchPriority: T
                }) : _ === "script" && r.d.X(y, {
                    crossOrigin: M,
                    integrity: j,
                    fetchPriority: T,
                    nonce: typeof p.nonce == "string" ? p.nonce : void 0
                });
            }
        }, ke.preinitModule = function(y, p) {
            if (typeof y == "string") if (typeof p == "object" && p !== null) {
                if (p.as == null || p.as === "script") {
                    var _ = v(p.as, p.crossOrigin);
                    r.d.M(y, {
                        crossOrigin: _,
                        integrity: typeof p.integrity == "string" ? p.integrity : void 0,
                        nonce: typeof p.nonce == "string" ? p.nonce : void 0
                    });
                }
            } else p == null && r.d.M(y);
        }, ke.preload = function(y, p) {
            if (typeof y == "string" && typeof p == "object" && p !== null && typeof p.as == "string") {
                var _ = p.as, M = v(_, p.crossOrigin);
                r.d.L(y, _, {
                    crossOrigin: M,
                    integrity: typeof p.integrity == "string" ? p.integrity : void 0,
                    nonce: typeof p.nonce == "string" ? p.nonce : void 0,
                    type: typeof p.type == "string" ? p.type : void 0,
                    fetchPriority: typeof p.fetchPriority == "string" ? p.fetchPriority : void 0,
                    referrerPolicy: typeof p.referrerPolicy == "string" ? p.referrerPolicy : void 0,
                    imageSrcSet: typeof p.imageSrcSet == "string" ? p.imageSrcSet : void 0,
                    imageSizes: typeof p.imageSizes == "string" ? p.imageSizes : void 0,
                    media: typeof p.media == "string" ? p.media : void 0
                });
            }
        }, ke.preloadModule = function(y, p) {
            if (typeof y == "string") if (p) {
                var _ = v(p.as, p.crossOrigin);
                r.d.m(y, {
                    as: typeof p.as == "string" && p.as !== "script" ? p.as : void 0,
                    crossOrigin: _,
                    integrity: typeof p.integrity == "string" ? p.integrity : void 0
                });
            } else r.d.m(y);
        }, ke.requestFormReset = function(y) {
            r.d.r(y);
        }, ke.unstable_batchedUpdates = function(y, p) {
            return y(p);
        }, ke.useFormState = function(y, p, _) {
            return m.H.useFormState(y, p, _);
        }, ke.useFormStatus = function() {
            return m.H.useHostTransitionStatus();
        }, ke.version = "19.2.1", ke;
    }
    var qh;
    function cy() {
        if (qh) return kr.exports;
        qh = 1;
        function l() {
            if (!(typeof __REACT_DEVTOOLS_GLOBAL_HOOK__ > "u" || typeof __REACT_DEVTOOLS_GLOBAL_HOOK__.checkDCE != "function")) try {
                __REACT_DEVTOOLS_GLOBAL_HOOK__.checkDCE(l);
            } catch (i) {
                console.error(i);
            }
        }
        return l(), kr.exports = oy(), kr.exports;
    }
    var Uh;
    function fy() {
        if (Uh) return Fa;
        Uh = 1;
        var l = ry(), i = So(), u = cy();
        function r(e) {
            var t = "https://react.dev/errors/" + e;
            if (1 < arguments.length) {
                t += "?args[]=" + encodeURIComponent(arguments[1]);
                for(var n = 2; n < arguments.length; n++)t += "&args[]=" + encodeURIComponent(arguments[n]);
            }
            return "Minified React error #" + e + "; visit " + t + " for the full message or use the non-minified dev environment for full errors and additional helpful warnings.";
        }
        function c(e) {
            return !(!e || e.nodeType !== 1 && e.nodeType !== 9 && e.nodeType !== 11);
        }
        function d(e) {
            var t = e, n = e;
            if (e.alternate) for(; t.return;)t = t.return;
            else {
                e = t;
                do t = e, (t.flags & 4098) !== 0 && (n = t.return), e = t.return;
                while (e);
            }
            return t.tag === 3 ? n : null;
        }
        function m(e) {
            if (e.tag === 13) {
                var t = e.memoizedState;
                if (t === null && (e = e.alternate, e !== null && (t = e.memoizedState)), t !== null) return t.dehydrated;
            }
            return null;
        }
        function v(e) {
            if (e.tag === 31) {
                var t = e.memoizedState;
                if (t === null && (e = e.alternate, e !== null && (t = e.memoizedState)), t !== null) return t.dehydrated;
            }
            return null;
        }
        function y(e) {
            if (d(e) !== e) throw Error(r(188));
        }
        function p(e) {
            var t = e.alternate;
            if (!t) {
                if (t = d(e), t === null) throw Error(r(188));
                return t !== e ? null : e;
            }
            for(var n = e, a = t;;){
                var s = n.return;
                if (s === null) break;
                var o = s.alternate;
                if (o === null) {
                    if (a = s.return, a !== null) {
                        n = a;
                        continue;
                    }
                    break;
                }
                if (s.child === o.child) {
                    for(o = s.child; o;){
                        if (o === n) return y(s), e;
                        if (o === a) return y(s), t;
                        o = o.sibling;
                    }
                    throw Error(r(188));
                }
                if (n.return !== a.return) n = s, a = o;
                else {
                    for(var f = !1, g = s.child; g;){
                        if (g === n) {
                            f = !0, n = s, a = o;
                            break;
                        }
                        if (g === a) {
                            f = !0, a = s, n = o;
                            break;
                        }
                        g = g.sibling;
                    }
                    if (!f) {
                        for(g = o.child; g;){
                            if (g === n) {
                                f = !0, n = o, a = s;
                                break;
                            }
                            if (g === a) {
                                f = !0, a = o, n = s;
                                break;
                            }
                            g = g.sibling;
                        }
                        if (!f) throw Error(r(189));
                    }
                }
                if (n.alternate !== a) throw Error(r(190));
            }
            if (n.tag !== 3) throw Error(r(188));
            return n.stateNode.current === n ? e : t;
        }
        function _(e) {
            var t = e.tag;
            if (t === 5 || t === 26 || t === 27 || t === 6) return e;
            for(e = e.child; e !== null;){
                if (t = _(e), t !== null) return t;
                e = e.sibling;
            }
            return null;
        }
        var M = Object.assign, j = Symbol.for("react.element"), T = Symbol.for("react.transitional.element"), U = Symbol.for("react.portal"), q = Symbol.for("react.fragment"), B = Symbol.for("react.strict_mode"), G = Symbol.for("react.profiler"), ae = Symbol.for("react.consumer"), Z = Symbol.for("react.context"), oe = Symbol.for("react.forward_ref"), he = Symbol.for("react.suspense"), F = Symbol.for("react.suspense_list"), $ = Symbol.for("react.memo"), I = Symbol.for("react.lazy"), Me = Symbol.for("react.activity"), Pe = Symbol.for("react.memo_cache_sentinel"), Be = Symbol.iterator;
        function tt(e) {
            return e === null || typeof e != "object" ? null : (e = Be && e[Be] || e["@@iterator"], typeof e == "function" ? e : null);
        }
        var Ln = Symbol.for("react.client.reference");
        function Qt(e) {
            if (e == null) return null;
            if (typeof e == "function") return e.$$typeof === Ln ? null : e.displayName || e.name || null;
            if (typeof e == "string") return e;
            switch(e){
                case q:
                    return "Fragment";
                case G:
                    return "Profiler";
                case B:
                    return "StrictMode";
                case he:
                    return "Suspense";
                case F:
                    return "SuspenseList";
                case Me:
                    return "Activity";
            }
            if (typeof e == "object") switch(e.$$typeof){
                case U:
                    return "Portal";
                case Z:
                    return e.displayName || "Context";
                case ae:
                    return (e._context.displayName || "Context") + ".Consumer";
                case oe:
                    var t = e.render;
                    return e = e.displayName, e || (e = t.displayName || t.name || "", e = e !== "" ? "ForwardRef(" + e + ")" : "ForwardRef"), e;
                case $:
                    return t = e.displayName || null, t !== null ? t : Qt(e.type) || "Memo";
                case I:
                    t = e._payload, e = e._init;
                    try {
                        return Qt(e(t));
                    } catch  {}
            }
            return null;
        }
        var At = Array.isArray, A = i.__CLIENT_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE, V = u.__DOM_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE, k = {
            pending: !1,
            data: null,
            method: null,
            action: null
        }, xe = [], we = -1;
        function x(e) {
            return {
                current: e
            };
        }
        function H(e) {
            0 > we || (e.current = xe[we], xe[we] = null, we--);
        }
        function L(e, t) {
            we++, xe[we] = e.current, e.current = t;
        }
        var Y = x(null), te = x(null), ue = x(null), ve = x(null);
        function Ie(e, t) {
            switch(L(ue, t), L(te, e), L(Y, null), t.nodeType){
                case 9:
                case 11:
                    e = (e = t.documentElement) && (e = e.namespaceURI) ? Pd(e) : 0;
                    break;
                default:
                    if (e = t.tagName, t = t.namespaceURI) t = Pd(t), e = Id(t, e);
                    else switch(e){
                        case "svg":
                            e = 1;
                            break;
                        case "math":
                            e = 2;
                            break;
                        default:
                            e = 0;
                    }
            }
            H(Y), L(Y, e);
        }
        function ze() {
            H(Y), H(te), H(ue);
        }
        function Wl(e) {
            e.memoizedState !== null && L(ve, e);
            var t = Y.current, n = Id(t, e.type);
            t !== n && (L(te, e), L(Y, n));
        }
        function ti(e) {
            te.current === e && (H(Y), H(te)), ve.current === e && (H(ve), Ga._currentValue = k);
        }
        var Mu, Mo;
        function Gn(e) {
            if (Mu === void 0) try {
                throw Error();
            } catch (n) {
                var t = n.stack.trim().match(/\n( *(at )?)/);
                Mu = t && t[1] || "", Mo = -1 < n.stack.indexOf(`
    at`) ? " (<anonymous>)" : -1 < n.stack.indexOf("@") ? "@unknown:0:0" : "";
            }
            return `
` + Mu + e + Mo;
        }
        var ju = !1;
        function Ou(e, t) {
            if (!e || ju) return "";
            ju = !0;
            var n = Error.prepareStackTrace;
            Error.prepareStackTrace = void 0;
            try {
                var a = {
                    DetermineComponentFrameRoot: function() {
                        try {
                            if (t) {
                                var D = function() {
                                    throw Error();
                                };
                                if (Object.defineProperty(D.prototype, "props", {
                                    set: function() {
                                        throw Error();
                                    }
                                }), typeof Reflect == "object" && Reflect.construct) {
                                    try {
                                        Reflect.construct(D, []);
                                    } catch (O) {
                                        var E = O;
                                    }
                                    Reflect.construct(e, [], D);
                                } else {
                                    try {
                                        D.call();
                                    } catch (O) {
                                        E = O;
                                    }
                                    e.call(D.prototype);
                                }
                            } else {
                                try {
                                    throw Error();
                                } catch (O) {
                                    E = O;
                                }
                                (D = e()) && typeof D.catch == "function" && D.catch(function() {});
                            }
                        } catch (O) {
                            if (O && E && typeof O.stack == "string") return [
                                O.stack,
                                E.stack
                            ];
                        }
                        return [
                            null,
                            null
                        ];
                    }
                };
                a.DetermineComponentFrameRoot.displayName = "DetermineComponentFrameRoot";
                var s = Object.getOwnPropertyDescriptor(a.DetermineComponentFrameRoot, "name");
                s && s.configurable && Object.defineProperty(a.DetermineComponentFrameRoot, "name", {
                    value: "DetermineComponentFrameRoot"
                });
                var o = a.DetermineComponentFrameRoot(), f = o[0], g = o[1];
                if (f && g) {
                    var S = f.split(`
`), w = g.split(`
`);
                    for(s = a = 0; a < S.length && !S[a].includes("DetermineComponentFrameRoot");)a++;
                    for(; s < w.length && !w[s].includes("DetermineComponentFrameRoot");)s++;
                    if (a === S.length || s === w.length) for(a = S.length - 1, s = w.length - 1; 1 <= a && 0 <= s && S[a] !== w[s];)s--;
                    for(; 1 <= a && 0 <= s; a--, s--)if (S[a] !== w[s]) {
                        if (a !== 1 || s !== 1) do if (a--, s--, 0 > s || S[a] !== w[s]) {
                            var N = `
` + S[a].replace(" at new ", " at ");
                            return e.displayName && N.includes("<anonymous>") && (N = N.replace("<anonymous>", e.displayName)), N;
                        }
                        while (1 <= a && 0 <= s);
                        break;
                    }
                }
            } finally{
                ju = !1, Error.prepareStackTrace = n;
            }
            return (n = e ? e.displayName || e.name : "") ? Gn(n) : "";
        }
        function Hg(e, t) {
            switch(e.tag){
                case 26:
                case 27:
                case 5:
                    return Gn(e.type);
                case 16:
                    return Gn("Lazy");
                case 13:
                    return e.child !== t && t !== null ? Gn("Suspense Fallback") : Gn("Suspense");
                case 19:
                    return Gn("SuspenseList");
                case 0:
                case 15:
                    return Ou(e.type, !1);
                case 11:
                    return Ou(e.type.render, !1);
                case 1:
                    return Ou(e.type, !0);
                case 31:
                    return Gn("Activity");
                default:
                    return "";
            }
        }
        function jo(e) {
            try {
                var t = "", n = null;
                do t += Hg(e, n), n = e, e = e.return;
                while (e);
                return t;
            } catch (a) {
                return `
Error generating stack: ` + a.message + `
` + a.stack;
            }
        }
        var Tu = Object.prototype.hasOwnProperty, Nu = l.unstable_scheduleCallback, Au = l.unstable_cancelCallback, qg = l.unstable_shouldYield, Ug = l.unstable_requestPaint, ct = l.unstable_now, Vg = l.unstable_getCurrentPriorityLevel, Oo = l.unstable_ImmediatePriority, To = l.unstable_UserBlockingPriority, ni = l.unstable_NormalPriority, Lg = l.unstable_LowPriority, No = l.unstable_IdlePriority, Gg = l.log, Bg = l.unstable_setDisableYieldValue, Pl = null, ft = null;
        function on(e) {
            if (typeof Gg == "function" && Bg(e), ft && typeof ft.setStrictMode == "function") try {
                ft.setStrictMode(Pl, e);
            } catch  {}
        }
        var dt = Math.clz32 ? Math.clz32 : Xg, Qg = Math.log, Yg = Math.LN2;
        function Xg(e) {
            return e >>>= 0, e === 0 ? 32 : 31 - (Qg(e) / Yg | 0) | 0;
        }
        var li = 256, ai = 262144, ii = 4194304;
        function Bn(e) {
            var t = e & 42;
            if (t !== 0) return t;
            switch(e & -e){
                case 1:
                    return 1;
                case 2:
                    return 2;
                case 4:
                    return 4;
                case 8:
                    return 8;
                case 16:
                    return 16;
                case 32:
                    return 32;
                case 64:
                    return 64;
                case 128:
                    return 128;
                case 256:
                case 512:
                case 1024:
                case 2048:
                case 4096:
                case 8192:
                case 16384:
                case 32768:
                case 65536:
                case 131072:
                    return e & 261888;
                case 262144:
                case 524288:
                case 1048576:
                case 2097152:
                    return e & 3932160;
                case 4194304:
                case 8388608:
                case 16777216:
                case 33554432:
                    return e & 62914560;
                case 67108864:
                    return 67108864;
                case 134217728:
                    return 134217728;
                case 268435456:
                    return 268435456;
                case 536870912:
                    return 536870912;
                case 1073741824:
                    return 0;
                default:
                    return e;
            }
        }
        function ui(e, t, n) {
            var a = e.pendingLanes;
            if (a === 0) return 0;
            var s = 0, o = e.suspendedLanes, f = e.pingedLanes;
            e = e.warmLanes;
            var g = a & 134217727;
            return g !== 0 ? (a = g & ~o, a !== 0 ? s = Bn(a) : (f &= g, f !== 0 ? s = Bn(f) : n || (n = g & ~e, n !== 0 && (s = Bn(n))))) : (g = a & ~o, g !== 0 ? s = Bn(g) : f !== 0 ? s = Bn(f) : n || (n = a & ~e, n !== 0 && (s = Bn(n)))), s === 0 ? 0 : t !== 0 && t !== s && (t & o) === 0 && (o = s & -s, n = t & -t, o >= n || o === 32 && (n & 4194048) !== 0) ? t : s;
        }
        function Il(e, t) {
            return (e.pendingLanes & ~(e.suspendedLanes & ~e.pingedLanes) & t) === 0;
        }
        function Fg(e, t) {
            switch(e){
                case 1:
                case 2:
                case 4:
                case 8:
                case 64:
                    return t + 250;
                case 16:
                case 32:
                case 128:
                case 256:
                case 512:
                case 1024:
                case 2048:
                case 4096:
                case 8192:
                case 16384:
                case 32768:
                case 65536:
                case 131072:
                case 262144:
                case 524288:
                case 1048576:
                case 2097152:
                    return t + 5e3;
                case 4194304:
                case 8388608:
                case 16777216:
                case 33554432:
                    return -1;
                case 67108864:
                case 134217728:
                case 268435456:
                case 536870912:
                case 1073741824:
                    return -1;
                default:
                    return -1;
            }
        }
        function Ao() {
            var e = ii;
            return ii <<= 1, (ii & 62914560) === 0 && (ii = 4194304), e;
        }
        function zu(e) {
            for(var t = [], n = 0; 31 > n; n++)t.push(e);
            return t;
        }
        function ea(e, t) {
            e.pendingLanes |= t, t !== 268435456 && (e.suspendedLanes = 0, e.pingedLanes = 0, e.warmLanes = 0);
        }
        function Zg(e, t, n, a, s, o) {
            var f = e.pendingLanes;
            e.pendingLanes = n, e.suspendedLanes = 0, e.pingedLanes = 0, e.warmLanes = 0, e.expiredLanes &= n, e.entangledLanes &= n, e.errorRecoveryDisabledLanes &= n, e.shellSuspendCounter = 0;
            var g = e.entanglements, S = e.expirationTimes, w = e.hiddenUpdates;
            for(n = f & ~n; 0 < n;){
                var N = 31 - dt(n), D = 1 << N;
                g[N] = 0, S[N] = -1;
                var E = w[N];
                if (E !== null) for(w[N] = null, N = 0; N < E.length; N++){
                    var O = E[N];
                    O !== null && (O.lane &= -536870913);
                }
                n &= ~D;
            }
            a !== 0 && zo(e, a, 0), o !== 0 && s === 0 && e.tag !== 0 && (e.suspendedLanes |= o & ~(f & ~t));
        }
        function zo(e, t, n) {
            e.pendingLanes |= t, e.suspendedLanes &= ~t;
            var a = 31 - dt(t);
            e.entangledLanes |= t, e.entanglements[a] = e.entanglements[a] | 1073741824 | n & 261930;
        }
        function Do(e, t) {
            var n = e.entangledLanes |= t;
            for(e = e.entanglements; n;){
                var a = 31 - dt(n), s = 1 << a;
                s & t | e[a] & t && (e[a] |= t), n &= ~s;
            }
        }
        function Ho(e, t) {
            var n = t & -t;
            return n = (n & 42) !== 0 ? 1 : Du(n), (n & (e.suspendedLanes | t)) !== 0 ? 0 : n;
        }
        function Du(e) {
            switch(e){
                case 2:
                    e = 1;
                    break;
                case 8:
                    e = 4;
                    break;
                case 32:
                    e = 16;
                    break;
                case 256:
                case 512:
                case 1024:
                case 2048:
                case 4096:
                case 8192:
                case 16384:
                case 32768:
                case 65536:
                case 131072:
                case 262144:
                case 524288:
                case 1048576:
                case 2097152:
                case 4194304:
                case 8388608:
                case 16777216:
                case 33554432:
                    e = 128;
                    break;
                case 268435456:
                    e = 134217728;
                    break;
                default:
                    e = 0;
            }
            return e;
        }
        function Hu(e) {
            return e &= -e, 2 < e ? 8 < e ? (e & 134217727) !== 0 ? 32 : 268435456 : 8 : 2;
        }
        function qo() {
            var e = V.p;
            return e !== 0 ? e : (e = window.event, e === void 0 ? 32 : _h(e.type));
        }
        function Uo(e, t) {
            var n = V.p;
            try {
                return V.p = e, t();
            } finally{
                V.p = n;
            }
        }
        var cn = Math.random().toString(36).slice(2), Xe = "__reactFiber$" + cn, nt = "__reactProps$" + cn, ol = "__reactContainer$" + cn, qu = "__reactEvents$" + cn, Kg = "__reactListeners$" + cn, $g = "__reactHandles$" + cn, Vo = "__reactResources$" + cn, ta = "__reactMarker$" + cn;
        function Uu(e) {
            delete e[Xe], delete e[nt], delete e[qu], delete e[Kg], delete e[$g];
        }
        function cl(e) {
            var t = e[Xe];
            if (t) return t;
            for(var n = e.parentNode; n;){
                if (t = n[ol] || n[Xe]) {
                    if (n = t.alternate, t.child !== null || n !== null && n.child !== null) for(e = uh(e); e !== null;){
                        if (n = e[Xe]) return n;
                        e = uh(e);
                    }
                    return t;
                }
                e = n, n = e.parentNode;
            }
            return null;
        }
        function fl(e) {
            if (e = e[Xe] || e[ol]) {
                var t = e.tag;
                if (t === 5 || t === 6 || t === 13 || t === 31 || t === 26 || t === 27 || t === 3) return e;
            }
            return null;
        }
        function na(e) {
            var t = e.tag;
            if (t === 5 || t === 26 || t === 27 || t === 6) return e.stateNode;
            throw Error(r(33));
        }
        function dl(e) {
            var t = e[Vo];
            return t || (t = e[Vo] = {
                hoistableStyles: new Map,
                hoistableScripts: new Map
            }), t;
        }
        function Qe(e) {
            e[ta] = !0;
        }
        var Lo = new Set, Go = {};
        function Qn(e, t) {
            hl(e, t), hl(e + "Capture", t);
        }
        function hl(e, t) {
            for(Go[e] = t, e = 0; e < t.length; e++)Lo.add(t[e]);
        }
        var Jg = RegExp("^[:A-Z_a-z\\u00C0-\\u00D6\\u00D8-\\u00F6\\u00F8-\\u02FF\\u0370-\\u037D\\u037F-\\u1FFF\\u200C-\\u200D\\u2070-\\u218F\\u2C00-\\u2FEF\\u3001-\\uD7FF\\uF900-\\uFDCF\\uFDF0-\\uFFFD][:A-Z_a-z\\u00C0-\\u00D6\\u00D8-\\u00F6\\u00F8-\\u02FF\\u0370-\\u037D\\u037F-\\u1FFF\\u200C-\\u200D\\u2070-\\u218F\\u2C00-\\u2FEF\\u3001-\\uD7FF\\uF900-\\uFDCF\\uFDF0-\\uFFFD\\-.0-9\\u00B7\\u0300-\\u036F\\u203F-\\u2040]*$"), Bo = {}, Qo = {};
        function kg(e) {
            return Tu.call(Qo, e) ? !0 : Tu.call(Bo, e) ? !1 : Jg.test(e) ? Qo[e] = !0 : (Bo[e] = !0, !1);
        }
        function si(e, t, n) {
            if (kg(t)) if (n === null) e.removeAttribute(t);
            else {
                switch(typeof n){
                    case "undefined":
                    case "function":
                    case "symbol":
                        e.removeAttribute(t);
                        return;
                    case "boolean":
                        var a = t.toLowerCase().slice(0, 5);
                        if (a !== "data-" && a !== "aria-") {
                            e.removeAttribute(t);
                            return;
                        }
                }
                e.setAttribute(t, "" + n);
            }
        }
        function ri(e, t, n) {
            if (n === null) e.removeAttribute(t);
            else {
                switch(typeof n){
                    case "undefined":
                    case "function":
                    case "symbol":
                    case "boolean":
                        e.removeAttribute(t);
                        return;
                }
                e.setAttribute(t, "" + n);
            }
        }
        function Yt(e, t, n, a) {
            if (a === null) e.removeAttribute(n);
            else {
                switch(typeof a){
                    case "undefined":
                    case "function":
                    case "symbol":
                    case "boolean":
                        e.removeAttribute(n);
                        return;
                }
                e.setAttributeNS(t, n, "" + a);
            }
        }
        function xt(e) {
            switch(typeof e){
                case "bigint":
                case "boolean":
                case "number":
                case "string":
                case "undefined":
                    return e;
                case "object":
                    return e;
                default:
                    return "";
            }
        }
        function Yo(e) {
            var t = e.type;
            return (e = e.nodeName) && e.toLowerCase() === "input" && (t === "checkbox" || t === "radio");
        }
        function Wg(e, t, n) {
            var a = Object.getOwnPropertyDescriptor(e.constructor.prototype, t);
            if (!e.hasOwnProperty(t) && typeof a < "u" && typeof a.get == "function" && typeof a.set == "function") {
                var s = a.get, o = a.set;
                return Object.defineProperty(e, t, {
                    configurable: !0,
                    get: function() {
                        return s.call(this);
                    },
                    set: function(f) {
                        n = "" + f, o.call(this, f);
                    }
                }), Object.defineProperty(e, t, {
                    enumerable: a.enumerable
                }), {
                    getValue: function() {
                        return n;
                    },
                    setValue: function(f) {
                        n = "" + f;
                    },
                    stopTracking: function() {
                        e._valueTracker = null, delete e[t];
                    }
                };
            }
        }
        function Vu(e) {
            if (!e._valueTracker) {
                var t = Yo(e) ? "checked" : "value";
                e._valueTracker = Wg(e, t, "" + e[t]);
            }
        }
        function Xo(e) {
            if (!e) return !1;
            var t = e._valueTracker;
            if (!t) return !0;
            var n = t.getValue(), a = "";
            return e && (a = Yo(e) ? e.checked ? "true" : "false" : e.value), e = a, e !== n ? (t.setValue(e), !0) : !1;
        }
        function oi(e) {
            if (e = e || (typeof document < "u" ? document : void 0), typeof e > "u") return null;
            try {
                return e.activeElement || e.body;
            } catch  {
                return e.body;
            }
        }
        var Pg = /[\n"\\]/g;
        function _t(e) {
            return e.replace(Pg, function(t) {
                return "\\" + t.charCodeAt(0).toString(16) + " ";
            });
        }
        function Lu(e, t, n, a, s, o, f, g) {
            e.name = "", f != null && typeof f != "function" && typeof f != "symbol" && typeof f != "boolean" ? e.type = f : e.removeAttribute("type"), t != null ? f === "number" ? (t === 0 && e.value === "" || e.value != t) && (e.value = "" + xt(t)) : e.value !== "" + xt(t) && (e.value = "" + xt(t)) : f !== "submit" && f !== "reset" || e.removeAttribute("value"), t != null ? Gu(e, f, xt(t)) : n != null ? Gu(e, f, xt(n)) : a != null && e.removeAttribute("value"), s == null && o != null && (e.defaultChecked = !!o), s != null && (e.checked = s && typeof s != "function" && typeof s != "symbol"), g != null && typeof g != "function" && typeof g != "symbol" && typeof g != "boolean" ? e.name = "" + xt(g) : e.removeAttribute("name");
        }
        function Fo(e, t, n, a, s, o, f, g) {
            if (o != null && typeof o != "function" && typeof o != "symbol" && typeof o != "boolean" && (e.type = o), t != null || n != null) {
                if (!(o !== "submit" && o !== "reset" || t != null)) {
                    Vu(e);
                    return;
                }
                n = n != null ? "" + xt(n) : "", t = t != null ? "" + xt(t) : n, g || t === e.value || (e.value = t), e.defaultValue = t;
            }
            a = a ?? s, a = typeof a != "function" && typeof a != "symbol" && !!a, e.checked = g ? e.checked : !!a, e.defaultChecked = !!a, f != null && typeof f != "function" && typeof f != "symbol" && typeof f != "boolean" && (e.name = f), Vu(e);
        }
        function Gu(e, t, n) {
            t === "number" && oi(e.ownerDocument) === e || e.defaultValue === "" + n || (e.defaultValue = "" + n);
        }
        function gl(e, t, n, a) {
            if (e = e.options, t) {
                t = {};
                for(var s = 0; s < n.length; s++)t["$" + n[s]] = !0;
                for(n = 0; n < e.length; n++)s = t.hasOwnProperty("$" + e[n].value), e[n].selected !== s && (e[n].selected = s), s && a && (e[n].defaultSelected = !0);
            } else {
                for(n = "" + xt(n), t = null, s = 0; s < e.length; s++){
                    if (e[s].value === n) {
                        e[s].selected = !0, a && (e[s].defaultSelected = !0);
                        return;
                    }
                    t !== null || e[s].disabled || (t = e[s]);
                }
                t !== null && (t.selected = !0);
            }
        }
        function Zo(e, t, n) {
            if (t != null && (t = "" + xt(t), t !== e.value && (e.value = t), n == null)) {
                e.defaultValue !== t && (e.defaultValue = t);
                return;
            }
            e.defaultValue = n != null ? "" + xt(n) : "";
        }
        function Ko(e, t, n, a) {
            if (t == null) {
                if (a != null) {
                    if (n != null) throw Error(r(92));
                    if (At(a)) {
                        if (1 < a.length) throw Error(r(93));
                        a = a[0];
                    }
                    n = a;
                }
                n == null && (n = ""), t = n;
            }
            n = xt(t), e.defaultValue = n, a = e.textContent, a === n && a !== "" && a !== null && (e.value = a), Vu(e);
        }
        function ml(e, t) {
            if (t) {
                var n = e.firstChild;
                if (n && n === e.lastChild && n.nodeType === 3) {
                    n.nodeValue = t;
                    return;
                }
            }
            e.textContent = t;
        }
        var Ig = new Set("animationIterationCount aspectRatio borderImageOutset borderImageSlice borderImageWidth boxFlex boxFlexGroup boxOrdinalGroup columnCount columns flex flexGrow flexPositive flexShrink flexNegative flexOrder gridArea gridRow gridRowEnd gridRowSpan gridRowStart gridColumn gridColumnEnd gridColumnSpan gridColumnStart fontWeight lineClamp lineHeight opacity order orphans scale tabSize widows zIndex zoom fillOpacity floodOpacity stopOpacity strokeDasharray strokeDashoffset strokeMiterlimit strokeOpacity strokeWidth MozAnimationIterationCount MozBoxFlex MozBoxFlexGroup MozLineClamp msAnimationIterationCount msFlex msZoom msFlexGrow msFlexNegative msFlexOrder msFlexPositive msFlexShrink msGridColumn msGridColumnSpan msGridRow msGridRowSpan WebkitAnimationIterationCount WebkitBoxFlex WebKitBoxFlexGroup WebkitBoxOrdinalGroup WebkitColumnCount WebkitColumns WebkitFlex WebkitFlexGrow WebkitFlexPositive WebkitFlexShrink WebkitLineClamp".split(" "));
        function $o(e, t, n) {
            var a = t.indexOf("--") === 0;
            n == null || typeof n == "boolean" || n === "" ? a ? e.setProperty(t, "") : t === "float" ? e.cssFloat = "" : e[t] = "" : a ? e.setProperty(t, n) : typeof n != "number" || n === 0 || Ig.has(t) ? t === "float" ? e.cssFloat = n : e[t] = ("" + n).trim() : e[t] = n + "px";
        }
        function Jo(e, t, n) {
            if (t != null && typeof t != "object") throw Error(r(62));
            if (e = e.style, n != null) {
                for(var a in n)!n.hasOwnProperty(a) || t != null && t.hasOwnProperty(a) || (a.indexOf("--") === 0 ? e.setProperty(a, "") : a === "float" ? e.cssFloat = "" : e[a] = "");
                for(var s in t)a = t[s], t.hasOwnProperty(s) && n[s] !== a && $o(e, s, a);
            } else for(var o in t)t.hasOwnProperty(o) && $o(e, o, t[o]);
        }
        function Bu(e) {
            if (e.indexOf("-") === -1) return !1;
            switch(e){
                case "annotation-xml":
                case "color-profile":
                case "font-face":
                case "font-face-src":
                case "font-face-uri":
                case "font-face-format":
                case "font-face-name":
                case "missing-glyph":
                    return !1;
                default:
                    return !0;
            }
        }
        var em = new Map([
            [
                "acceptCharset",
                "accept-charset"
            ],
            [
                "htmlFor",
                "for"
            ],
            [
                "httpEquiv",
                "http-equiv"
            ],
            [
                "crossOrigin",
                "crossorigin"
            ],
            [
                "accentHeight",
                "accent-height"
            ],
            [
                "alignmentBaseline",
                "alignment-baseline"
            ],
            [
                "arabicForm",
                "arabic-form"
            ],
            [
                "baselineShift",
                "baseline-shift"
            ],
            [
                "capHeight",
                "cap-height"
            ],
            [
                "clipPath",
                "clip-path"
            ],
            [
                "clipRule",
                "clip-rule"
            ],
            [
                "colorInterpolation",
                "color-interpolation"
            ],
            [
                "colorInterpolationFilters",
                "color-interpolation-filters"
            ],
            [
                "colorProfile",
                "color-profile"
            ],
            [
                "colorRendering",
                "color-rendering"
            ],
            [
                "dominantBaseline",
                "dominant-baseline"
            ],
            [
                "enableBackground",
                "enable-background"
            ],
            [
                "fillOpacity",
                "fill-opacity"
            ],
            [
                "fillRule",
                "fill-rule"
            ],
            [
                "floodColor",
                "flood-color"
            ],
            [
                "floodOpacity",
                "flood-opacity"
            ],
            [
                "fontFamily",
                "font-family"
            ],
            [
                "fontSize",
                "font-size"
            ],
            [
                "fontSizeAdjust",
                "font-size-adjust"
            ],
            [
                "fontStretch",
                "font-stretch"
            ],
            [
                "fontStyle",
                "font-style"
            ],
            [
                "fontVariant",
                "font-variant"
            ],
            [
                "fontWeight",
                "font-weight"
            ],
            [
                "glyphName",
                "glyph-name"
            ],
            [
                "glyphOrientationHorizontal",
                "glyph-orientation-horizontal"
            ],
            [
                "glyphOrientationVertical",
                "glyph-orientation-vertical"
            ],
            [
                "horizAdvX",
                "horiz-adv-x"
            ],
            [
                "horizOriginX",
                "horiz-origin-x"
            ],
            [
                "imageRendering",
                "image-rendering"
            ],
            [
                "letterSpacing",
                "letter-spacing"
            ],
            [
                "lightingColor",
                "lighting-color"
            ],
            [
                "markerEnd",
                "marker-end"
            ],
            [
                "markerMid",
                "marker-mid"
            ],
            [
                "markerStart",
                "marker-start"
            ],
            [
                "overlinePosition",
                "overline-position"
            ],
            [
                "overlineThickness",
                "overline-thickness"
            ],
            [
                "paintOrder",
                "paint-order"
            ],
            [
                "panose-1",
                "panose-1"
            ],
            [
                "pointerEvents",
                "pointer-events"
            ],
            [
                "renderingIntent",
                "rendering-intent"
            ],
            [
                "shapeRendering",
                "shape-rendering"
            ],
            [
                "stopColor",
                "stop-color"
            ],
            [
                "stopOpacity",
                "stop-opacity"
            ],
            [
                "strikethroughPosition",
                "strikethrough-position"
            ],
            [
                "strikethroughThickness",
                "strikethrough-thickness"
            ],
            [
                "strokeDasharray",
                "stroke-dasharray"
            ],
            [
                "strokeDashoffset",
                "stroke-dashoffset"
            ],
            [
                "strokeLinecap",
                "stroke-linecap"
            ],
            [
                "strokeLinejoin",
                "stroke-linejoin"
            ],
            [
                "strokeMiterlimit",
                "stroke-miterlimit"
            ],
            [
                "strokeOpacity",
                "stroke-opacity"
            ],
            [
                "strokeWidth",
                "stroke-width"
            ],
            [
                "textAnchor",
                "text-anchor"
            ],
            [
                "textDecoration",
                "text-decoration"
            ],
            [
                "textRendering",
                "text-rendering"
            ],
            [
                "transformOrigin",
                "transform-origin"
            ],
            [
                "underlinePosition",
                "underline-position"
            ],
            [
                "underlineThickness",
                "underline-thickness"
            ],
            [
                "unicodeBidi",
                "unicode-bidi"
            ],
            [
                "unicodeRange",
                "unicode-range"
            ],
            [
                "unitsPerEm",
                "units-per-em"
            ],
            [
                "vAlphabetic",
                "v-alphabetic"
            ],
            [
                "vHanging",
                "v-hanging"
            ],
            [
                "vIdeographic",
                "v-ideographic"
            ],
            [
                "vMathematical",
                "v-mathematical"
            ],
            [
                "vectorEffect",
                "vector-effect"
            ],
            [
                "vertAdvY",
                "vert-adv-y"
            ],
            [
                "vertOriginX",
                "vert-origin-x"
            ],
            [
                "vertOriginY",
                "vert-origin-y"
            ],
            [
                "wordSpacing",
                "word-spacing"
            ],
            [
                "writingMode",
                "writing-mode"
            ],
            [
                "xmlnsXlink",
                "xmlns:xlink"
            ],
            [
                "xHeight",
                "x-height"
            ]
        ]), tm = /^[\u0000-\u001F ]*j[\r\n\t]*a[\r\n\t]*v[\r\n\t]*a[\r\n\t]*s[\r\n\t]*c[\r\n\t]*r[\r\n\t]*i[\r\n\t]*p[\r\n\t]*t[\r\n\t]*:/i;
        function ci(e) {
            return tm.test("" + e) ? "javascript:throw new Error('React has blocked a javascript: URL as a security precaution.')" : e;
        }
        function Xt() {}
        var Qu = null;
        function Yu(e) {
            return e = e.target || e.srcElement || window, e.correspondingUseElement && (e = e.correspondingUseElement), e.nodeType === 3 ? e.parentNode : e;
        }
        var yl = null, vl = null;
        function ko(e) {
            var t = fl(e);
            if (t && (e = t.stateNode)) {
                var n = e[nt] || null;
                e: switch(e = t.stateNode, t.type){
                    case "input":
                        if (Lu(e, n.value, n.defaultValue, n.defaultValue, n.checked, n.defaultChecked, n.type, n.name), t = n.name, n.type === "radio" && t != null) {
                            for(n = e; n.parentNode;)n = n.parentNode;
                            for(n = n.querySelectorAll('input[name="' + _t("" + t) + '"][type="radio"]'), t = 0; t < n.length; t++){
                                var a = n[t];
                                if (a !== e && a.form === e.form) {
                                    var s = a[nt] || null;
                                    if (!s) throw Error(r(90));
                                    Lu(a, s.value, s.defaultValue, s.defaultValue, s.checked, s.defaultChecked, s.type, s.name);
                                }
                            }
                            for(t = 0; t < n.length; t++)a = n[t], a.form === e.form && Xo(a);
                        }
                        break e;
                    case "textarea":
                        Zo(e, n.value, n.defaultValue);
                        break e;
                    case "select":
                        t = n.value, t != null && gl(e, !!n.multiple, t, !1);
                }
            }
        }
        var Xu = !1;
        function Wo(e, t, n) {
            if (Xu) return e(t, n);
            Xu = !0;
            try {
                var a = e(t);
                return a;
            } finally{
                if (Xu = !1, (yl !== null || vl !== null) && (Wi(), yl && (t = yl, e = vl, vl = yl = null, ko(t), e))) for(t = 0; t < e.length; t++)ko(e[t]);
            }
        }
        function la(e, t) {
            var n = e.stateNode;
            if (n === null) return null;
            var a = n[nt] || null;
            if (a === null) return null;
            n = a[t];
            e: switch(t){
                case "onClick":
                case "onClickCapture":
                case "onDoubleClick":
                case "onDoubleClickCapture":
                case "onMouseDown":
                case "onMouseDownCapture":
                case "onMouseMove":
                case "onMouseMoveCapture":
                case "onMouseUp":
                case "onMouseUpCapture":
                case "onMouseEnter":
                    (a = !a.disabled) || (e = e.type, a = !(e === "button" || e === "input" || e === "select" || e === "textarea")), e = !a;
                    break e;
                default:
                    e = !1;
            }
            if (e) return null;
            if (n && typeof n != "function") throw Error(r(231, t, typeof n));
            return n;
        }
        var Ft = !(typeof window > "u" || typeof window.document > "u" || typeof window.document.createElement > "u"), Fu = !1;
        if (Ft) try {
            var aa = {};
            Object.defineProperty(aa, "passive", {
                get: function() {
                    Fu = !0;
                }
            }), window.addEventListener("test", aa, aa), window.removeEventListener("test", aa, aa);
        } catch  {
            Fu = !1;
        }
        var fn = null, Zu = null, fi = null;
        function Po() {
            if (fi) return fi;
            var e, t = Zu, n = t.length, a, s = "value" in fn ? fn.value : fn.textContent, o = s.length;
            for(e = 0; e < n && t[e] === s[e]; e++);
            var f = n - e;
            for(a = 1; a <= f && t[n - a] === s[o - a]; a++);
            return fi = s.slice(e, 1 < a ? 1 - a : void 0);
        }
        function di(e) {
            var t = e.keyCode;
            return "charCode" in e ? (e = e.charCode, e === 0 && t === 13 && (e = 13)) : e = t, e === 10 && (e = 13), 32 <= e || e === 13 ? e : 0;
        }
        function hi() {
            return !0;
        }
        function Io() {
            return !1;
        }
        function lt(e) {
            function t(n, a, s, o, f) {
                this._reactName = n, this._targetInst = s, this.type = a, this.nativeEvent = o, this.target = f, this.currentTarget = null;
                for(var g in e)e.hasOwnProperty(g) && (n = e[g], this[g] = n ? n(o) : o[g]);
                return this.isDefaultPrevented = (o.defaultPrevented != null ? o.defaultPrevented : o.returnValue === !1) ? hi : Io, this.isPropagationStopped = Io, this;
            }
            return M(t.prototype, {
                preventDefault: function() {
                    this.defaultPrevented = !0;
                    var n = this.nativeEvent;
                    n && (n.preventDefault ? n.preventDefault() : typeof n.returnValue != "unknown" && (n.returnValue = !1), this.isDefaultPrevented = hi);
                },
                stopPropagation: function() {
                    var n = this.nativeEvent;
                    n && (n.stopPropagation ? n.stopPropagation() : typeof n.cancelBubble != "unknown" && (n.cancelBubble = !0), this.isPropagationStopped = hi);
                },
                persist: function() {},
                isPersistent: hi
            }), t;
        }
        var Yn = {
            eventPhase: 0,
            bubbles: 0,
            cancelable: 0,
            timeStamp: function(e) {
                return e.timeStamp || Date.now();
            },
            defaultPrevented: 0,
            isTrusted: 0
        }, gi = lt(Yn), ia = M({}, Yn, {
            view: 0,
            detail: 0
        }), nm = lt(ia), Ku, $u, ua, mi = M({}, ia, {
            screenX: 0,
            screenY: 0,
            clientX: 0,
            clientY: 0,
            pageX: 0,
            pageY: 0,
            ctrlKey: 0,
            shiftKey: 0,
            altKey: 0,
            metaKey: 0,
            getModifierState: ku,
            button: 0,
            buttons: 0,
            relatedTarget: function(e) {
                return e.relatedTarget === void 0 ? e.fromElement === e.srcElement ? e.toElement : e.fromElement : e.relatedTarget;
            },
            movementX: function(e) {
                return "movementX" in e ? e.movementX : (e !== ua && (ua && e.type === "mousemove" ? (Ku = e.screenX - ua.screenX, $u = e.screenY - ua.screenY) : $u = Ku = 0, ua = e), Ku);
            },
            movementY: function(e) {
                return "movementY" in e ? e.movementY : $u;
            }
        }), ec = lt(mi), lm = M({}, mi, {
            dataTransfer: 0
        }), am = lt(lm), im = M({}, ia, {
            relatedTarget: 0
        }), Ju = lt(im), um = M({}, Yn, {
            animationName: 0,
            elapsedTime: 0,
            pseudoElement: 0
        }), sm = lt(um), rm = M({}, Yn, {
            clipboardData: function(e) {
                return "clipboardData" in e ? e.clipboardData : window.clipboardData;
            }
        }), om = lt(rm), cm = M({}, Yn, {
            data: 0
        }), tc = lt(cm), fm = {
            Esc: "Escape",
            Spacebar: " ",
            Left: "ArrowLeft",
            Up: "ArrowUp",
            Right: "ArrowRight",
            Down: "ArrowDown",
            Del: "Delete",
            Win: "OS",
            Menu: "ContextMenu",
            Apps: "ContextMenu",
            Scroll: "ScrollLock",
            MozPrintableKey: "Unidentified"
        }, dm = {
            8: "Backspace",
            9: "Tab",
            12: "Clear",
            13: "Enter",
            16: "Shift",
            17: "Control",
            18: "Alt",
            19: "Pause",
            20: "CapsLock",
            27: "Escape",
            32: " ",
            33: "PageUp",
            34: "PageDown",
            35: "End",
            36: "Home",
            37: "ArrowLeft",
            38: "ArrowUp",
            39: "ArrowRight",
            40: "ArrowDown",
            45: "Insert",
            46: "Delete",
            112: "F1",
            113: "F2",
            114: "F3",
            115: "F4",
            116: "F5",
            117: "F6",
            118: "F7",
            119: "F8",
            120: "F9",
            121: "F10",
            122: "F11",
            123: "F12",
            144: "NumLock",
            145: "ScrollLock",
            224: "Meta"
        }, hm = {
            Alt: "altKey",
            Control: "ctrlKey",
            Meta: "metaKey",
            Shift: "shiftKey"
        };
        function gm(e) {
            var t = this.nativeEvent;
            return t.getModifierState ? t.getModifierState(e) : (e = hm[e]) ? !!t[e] : !1;
        }
        function ku() {
            return gm;
        }
        var mm = M({}, ia, {
            key: function(e) {
                if (e.key) {
                    var t = fm[e.key] || e.key;
                    if (t !== "Unidentified") return t;
                }
                return e.type === "keypress" ? (e = di(e), e === 13 ? "Enter" : String.fromCharCode(e)) : e.type === "keydown" || e.type === "keyup" ? dm[e.keyCode] || "Unidentified" : "";
            },
            code: 0,
            location: 0,
            ctrlKey: 0,
            shiftKey: 0,
            altKey: 0,
            metaKey: 0,
            repeat: 0,
            locale: 0,
            getModifierState: ku,
            charCode: function(e) {
                return e.type === "keypress" ? di(e) : 0;
            },
            keyCode: function(e) {
                return e.type === "keydown" || e.type === "keyup" ? e.keyCode : 0;
            },
            which: function(e) {
                return e.type === "keypress" ? di(e) : e.type === "keydown" || e.type === "keyup" ? e.keyCode : 0;
            }
        }), ym = lt(mm), vm = M({}, mi, {
            pointerId: 0,
            width: 0,
            height: 0,
            pressure: 0,
            tangentialPressure: 0,
            tiltX: 0,
            tiltY: 0,
            twist: 0,
            pointerType: 0,
            isPrimary: 0
        }), nc = lt(vm), pm = M({}, ia, {
            touches: 0,
            targetTouches: 0,
            changedTouches: 0,
            altKey: 0,
            metaKey: 0,
            ctrlKey: 0,
            shiftKey: 0,
            getModifierState: ku
        }), Sm = lt(pm), bm = M({}, Yn, {
            propertyName: 0,
            elapsedTime: 0,
            pseudoElement: 0
        }), xm = lt(bm), _m = M({}, mi, {
            deltaX: function(e) {
                return "deltaX" in e ? e.deltaX : "wheelDeltaX" in e ? -e.wheelDeltaX : 0;
            },
            deltaY: function(e) {
                return "deltaY" in e ? e.deltaY : "wheelDeltaY" in e ? -e.wheelDeltaY : "wheelDelta" in e ? -e.wheelDelta : 0;
            },
            deltaZ: 0,
            deltaMode: 0
        }), Cm = lt(_m), Rm = M({}, Yn, {
            newState: 0,
            oldState: 0
        }), wm = lt(Rm), Em = [
            9,
            13,
            27,
            32
        ], Wu = Ft && "CompositionEvent" in window, sa = null;
        Ft && "documentMode" in document && (sa = document.documentMode);
        var Mm = Ft && "TextEvent" in window && !sa, lc = Ft && (!Wu || sa && 8 < sa && 11 >= sa), ac = " ", ic = !1;
        function uc(e, t) {
            switch(e){
                case "keyup":
                    return Em.indexOf(t.keyCode) !== -1;
                case "keydown":
                    return t.keyCode !== 229;
                case "keypress":
                case "mousedown":
                case "focusout":
                    return !0;
                default:
                    return !1;
            }
        }
        function sc(e) {
            return e = e.detail, typeof e == "object" && "data" in e ? e.data : null;
        }
        var pl = !1;
        function jm(e, t) {
            switch(e){
                case "compositionend":
                    return sc(t);
                case "keypress":
                    return t.which !== 32 ? null : (ic = !0, ac);
                case "textInput":
                    return e = t.data, e === ac && ic ? null : e;
                default:
                    return null;
            }
        }
        function Om(e, t) {
            if (pl) return e === "compositionend" || !Wu && uc(e, t) ? (e = Po(), fi = Zu = fn = null, pl = !1, e) : null;
            switch(e){
                case "paste":
                    return null;
                case "keypress":
                    if (!(t.ctrlKey || t.altKey || t.metaKey) || t.ctrlKey && t.altKey) {
                        if (t.char && 1 < t.char.length) return t.char;
                        if (t.which) return String.fromCharCode(t.which);
                    }
                    return null;
                case "compositionend":
                    return lc && t.locale !== "ko" ? null : t.data;
                default:
                    return null;
            }
        }
        var Tm = {
            color: !0,
            date: !0,
            datetime: !0,
            "datetime-local": !0,
            email: !0,
            month: !0,
            number: !0,
            password: !0,
            range: !0,
            search: !0,
            tel: !0,
            text: !0,
            time: !0,
            url: !0,
            week: !0
        };
        function rc(e) {
            var t = e && e.nodeName && e.nodeName.toLowerCase();
            return t === "input" ? !!Tm[e.type] : t === "textarea";
        }
        function oc(e, t, n, a) {
            yl ? vl ? vl.push(a) : vl = [
                a
            ] : yl = a, t = au(t, "onChange"), 0 < t.length && (n = new gi("onChange", "change", null, n, a), e.push({
                event: n,
                listeners: t
            }));
        }
        var ra = null, oa = null;
        function Nm(e) {
            Zd(e, 0);
        }
        function yi(e) {
            var t = na(e);
            if (Xo(t)) return e;
        }
        function cc(e, t) {
            if (e === "change") return t;
        }
        var fc = !1;
        if (Ft) {
            var Pu;
            if (Ft) {
                var Iu = "oninput" in document;
                if (!Iu) {
                    var dc = document.createElement("div");
                    dc.setAttribute("oninput", "return;"), Iu = typeof dc.oninput == "function";
                }
                Pu = Iu;
            } else Pu = !1;
            fc = Pu && (!document.documentMode || 9 < document.documentMode);
        }
        function hc() {
            ra && (ra.detachEvent("onpropertychange", gc), oa = ra = null);
        }
        function gc(e) {
            if (e.propertyName === "value" && yi(oa)) {
                var t = [];
                oc(t, oa, e, Yu(e)), Wo(Nm, t);
            }
        }
        function Am(e, t, n) {
            e === "focusin" ? (hc(), ra = t, oa = n, ra.attachEvent("onpropertychange", gc)) : e === "focusout" && hc();
        }
        function zm(e) {
            if (e === "selectionchange" || e === "keyup" || e === "keydown") return yi(oa);
        }
        function Dm(e, t) {
            if (e === "click") return yi(t);
        }
        function Hm(e, t) {
            if (e === "input" || e === "change") return yi(t);
        }
        function qm(e, t) {
            return e === t && (e !== 0 || 1 / e === 1 / t) || e !== e && t !== t;
        }
        var ht = typeof Object.is == "function" ? Object.is : qm;
        function ca(e, t) {
            if (ht(e, t)) return !0;
            if (typeof e != "object" || e === null || typeof t != "object" || t === null) return !1;
            var n = Object.keys(e), a = Object.keys(t);
            if (n.length !== a.length) return !1;
            for(a = 0; a < n.length; a++){
                var s = n[a];
                if (!Tu.call(t, s) || !ht(e[s], t[s])) return !1;
            }
            return !0;
        }
        function mc(e) {
            for(; e && e.firstChild;)e = e.firstChild;
            return e;
        }
        function yc(e, t) {
            var n = mc(e);
            e = 0;
            for(var a; n;){
                if (n.nodeType === 3) {
                    if (a = e + n.textContent.length, e <= t && a >= t) return {
                        node: n,
                        offset: t - e
                    };
                    e = a;
                }
                e: {
                    for(; n;){
                        if (n.nextSibling) {
                            n = n.nextSibling;
                            break e;
                        }
                        n = n.parentNode;
                    }
                    n = void 0;
                }
                n = mc(n);
            }
        }
        function vc(e, t) {
            return e && t ? e === t ? !0 : e && e.nodeType === 3 ? !1 : t && t.nodeType === 3 ? vc(e, t.parentNode) : "contains" in e ? e.contains(t) : e.compareDocumentPosition ? !!(e.compareDocumentPosition(t) & 16) : !1 : !1;
        }
        function pc(e) {
            e = e != null && e.ownerDocument != null && e.ownerDocument.defaultView != null ? e.ownerDocument.defaultView : window;
            for(var t = oi(e.document); t instanceof e.HTMLIFrameElement;){
                try {
                    var n = typeof t.contentWindow.location.href == "string";
                } catch  {
                    n = !1;
                }
                if (n) e = t.contentWindow;
                else break;
                t = oi(e.document);
            }
            return t;
        }
        function es(e) {
            var t = e && e.nodeName && e.nodeName.toLowerCase();
            return t && (t === "input" && (e.type === "text" || e.type === "search" || e.type === "tel" || e.type === "url" || e.type === "password") || t === "textarea" || e.contentEditable === "true");
        }
        var Um = Ft && "documentMode" in document && 11 >= document.documentMode, Sl = null, ts = null, fa = null, ns = !1;
        function Sc(e, t, n) {
            var a = n.window === n ? n.document : n.nodeType === 9 ? n : n.ownerDocument;
            ns || Sl == null || Sl !== oi(a) || (a = Sl, "selectionStart" in a && es(a) ? a = {
                start: a.selectionStart,
                end: a.selectionEnd
            } : (a = (a.ownerDocument && a.ownerDocument.defaultView || window).getSelection(), a = {
                anchorNode: a.anchorNode,
                anchorOffset: a.anchorOffset,
                focusNode: a.focusNode,
                focusOffset: a.focusOffset
            }), fa && ca(fa, a) || (fa = a, a = au(ts, "onSelect"), 0 < a.length && (t = new gi("onSelect", "select", null, t, n), e.push({
                event: t,
                listeners: a
            }), t.target = Sl)));
        }
        function Xn(e, t) {
            var n = {};
            return n[e.toLowerCase()] = t.toLowerCase(), n["Webkit" + e] = "webkit" + t, n["Moz" + e] = "moz" + t, n;
        }
        var bl = {
            animationend: Xn("Animation", "AnimationEnd"),
            animationiteration: Xn("Animation", "AnimationIteration"),
            animationstart: Xn("Animation", "AnimationStart"),
            transitionrun: Xn("Transition", "TransitionRun"),
            transitionstart: Xn("Transition", "TransitionStart"),
            transitioncancel: Xn("Transition", "TransitionCancel"),
            transitionend: Xn("Transition", "TransitionEnd")
        }, ls = {}, bc = {};
        Ft && (bc = document.createElement("div").style, "AnimationEvent" in window || (delete bl.animationend.animation, delete bl.animationiteration.animation, delete bl.animationstart.animation), "TransitionEvent" in window || delete bl.transitionend.transition);
        function Fn(e) {
            if (ls[e]) return ls[e];
            if (!bl[e]) return e;
            var t = bl[e], n;
            for(n in t)if (t.hasOwnProperty(n) && n in bc) return ls[e] = t[n];
            return e;
        }
        var xc = Fn("animationend"), _c = Fn("animationiteration"), Cc = Fn("animationstart"), Vm = Fn("transitionrun"), Lm = Fn("transitionstart"), Gm = Fn("transitioncancel"), Rc = Fn("transitionend"), wc = new Map, as = "abort auxClick beforeToggle cancel canPlay canPlayThrough click close contextMenu copy cut drag dragEnd dragEnter dragExit dragLeave dragOver dragStart drop durationChange emptied encrypted ended error gotPointerCapture input invalid keyDown keyPress keyUp load loadedData loadedMetadata loadStart lostPointerCapture mouseDown mouseMove mouseOut mouseOver mouseUp paste pause play playing pointerCancel pointerDown pointerMove pointerOut pointerOver pointerUp progress rateChange reset resize seeked seeking stalled submit suspend timeUpdate touchCancel touchEnd touchStart volumeChange scroll toggle touchMove waiting wheel".split(" ");
        as.push("scrollEnd");
        function zt(e, t) {
            wc.set(e, t), Qn(t, [
                e
            ]);
        }
        var vi = typeof reportError == "function" ? reportError : function(e) {
            if (typeof window == "object" && typeof window.ErrorEvent == "function") {
                var t = new window.ErrorEvent("error", {
                    bubbles: !0,
                    cancelable: !0,
                    message: typeof e == "object" && e !== null && typeof e.message == "string" ? String(e.message) : String(e),
                    error: e
                });
                if (!window.dispatchEvent(t)) return;
            } else if (typeof process == "object" && typeof process.emit == "function") {
                process.emit("uncaughtException", e);
                return;
            }
            console.error(e);
        }, Ct = [], xl = 0, is = 0;
        function pi() {
            for(var e = xl, t = is = xl = 0; t < e;){
                var n = Ct[t];
                Ct[t++] = null;
                var a = Ct[t];
                Ct[t++] = null;
                var s = Ct[t];
                Ct[t++] = null;
                var o = Ct[t];
                if (Ct[t++] = null, a !== null && s !== null) {
                    var f = a.pending;
                    f === null ? s.next = s : (s.next = f.next, f.next = s), a.pending = s;
                }
                o !== 0 && Ec(n, s, o);
            }
        }
        function Si(e, t, n, a) {
            Ct[xl++] = e, Ct[xl++] = t, Ct[xl++] = n, Ct[xl++] = a, is |= a, e.lanes |= a, e = e.alternate, e !== null && (e.lanes |= a);
        }
        function us(e, t, n, a) {
            return Si(e, t, n, a), bi(e);
        }
        function Zn(e, t) {
            return Si(e, null, null, t), bi(e);
        }
        function Ec(e, t, n) {
            e.lanes |= n;
            var a = e.alternate;
            a !== null && (a.lanes |= n);
            for(var s = !1, o = e.return; o !== null;)o.childLanes |= n, a = o.alternate, a !== null && (a.childLanes |= n), o.tag === 22 && (e = o.stateNode, e === null || e._visibility & 1 || (s = !0)), e = o, o = o.return;
            return e.tag === 3 ? (o = e.stateNode, s && t !== null && (s = 31 - dt(n), e = o.hiddenUpdates, a = e[s], a === null ? e[s] = [
                t
            ] : a.push(t), t.lane = n | 536870912), o) : null;
        }
        function bi(e) {
            if (50 < za) throw za = 0, mr = null, Error(r(185));
            for(var t = e.return; t !== null;)e = t, t = e.return;
            return e.tag === 3 ? e.stateNode : null;
        }
        var _l = {};
        function Bm(e, t, n, a) {
            this.tag = e, this.key = n, this.sibling = this.child = this.return = this.stateNode = this.type = this.elementType = null, this.index = 0, this.refCleanup = this.ref = null, this.pendingProps = t, this.dependencies = this.memoizedState = this.updateQueue = this.memoizedProps = null, this.mode = a, this.subtreeFlags = this.flags = 0, this.deletions = null, this.childLanes = this.lanes = 0, this.alternate = null;
        }
        function gt(e, t, n, a) {
            return new Bm(e, t, n, a);
        }
        function ss(e) {
            return e = e.prototype, !(!e || !e.isReactComponent);
        }
        function Zt(e, t) {
            var n = e.alternate;
            return n === null ? (n = gt(e.tag, t, e.key, e.mode), n.elementType = e.elementType, n.type = e.type, n.stateNode = e.stateNode, n.alternate = e, e.alternate = n) : (n.pendingProps = t, n.type = e.type, n.flags = 0, n.subtreeFlags = 0, n.deletions = null), n.flags = e.flags & 65011712, n.childLanes = e.childLanes, n.lanes = e.lanes, n.child = e.child, n.memoizedProps = e.memoizedProps, n.memoizedState = e.memoizedState, n.updateQueue = e.updateQueue, t = e.dependencies, n.dependencies = t === null ? null : {
                lanes: t.lanes,
                firstContext: t.firstContext
            }, n.sibling = e.sibling, n.index = e.index, n.ref = e.ref, n.refCleanup = e.refCleanup, n;
        }
        function Mc(e, t) {
            e.flags &= 65011714;
            var n = e.alternate;
            return n === null ? (e.childLanes = 0, e.lanes = t, e.child = null, e.subtreeFlags = 0, e.memoizedProps = null, e.memoizedState = null, e.updateQueue = null, e.dependencies = null, e.stateNode = null) : (e.childLanes = n.childLanes, e.lanes = n.lanes, e.child = n.child, e.subtreeFlags = 0, e.deletions = null, e.memoizedProps = n.memoizedProps, e.memoizedState = n.memoizedState, e.updateQueue = n.updateQueue, e.type = n.type, t = n.dependencies, e.dependencies = t === null ? null : {
                lanes: t.lanes,
                firstContext: t.firstContext
            }), e;
        }
        function xi(e, t, n, a, s, o) {
            var f = 0;
            if (a = e, typeof e == "function") ss(e) && (f = 1);
            else if (typeof e == "string") f = Z0(e, n, Y.current) ? 26 : e === "html" || e === "head" || e === "body" ? 27 : 5;
            else e: switch(e){
                case Me:
                    return e = gt(31, n, t, s), e.elementType = Me, e.lanes = o, e;
                case q:
                    return Kn(n.children, s, o, t);
                case B:
                    f = 8, s |= 24;
                    break;
                case G:
                    return e = gt(12, n, t, s | 2), e.elementType = G, e.lanes = o, e;
                case he:
                    return e = gt(13, n, t, s), e.elementType = he, e.lanes = o, e;
                case F:
                    return e = gt(19, n, t, s), e.elementType = F, e.lanes = o, e;
                default:
                    if (typeof e == "object" && e !== null) switch(e.$$typeof){
                        case Z:
                            f = 10;
                            break e;
                        case ae:
                            f = 9;
                            break e;
                        case oe:
                            f = 11;
                            break e;
                        case $:
                            f = 14;
                            break e;
                        case I:
                            f = 16, a = null;
                            break e;
                    }
                    f = 29, n = Error(r(130, e === null ? "null" : typeof e, "")), a = null;
            }
            return t = gt(f, n, t, s), t.elementType = e, t.type = a, t.lanes = o, t;
        }
        function Kn(e, t, n, a) {
            return e = gt(7, e, a, t), e.lanes = n, e;
        }
        function rs(e, t, n) {
            return e = gt(6, e, null, t), e.lanes = n, e;
        }
        function jc(e) {
            var t = gt(18, null, null, 0);
            return t.stateNode = e, t;
        }
        function os(e, t, n) {
            return t = gt(4, e.children !== null ? e.children : [], e.key, t), t.lanes = n, t.stateNode = {
                containerInfo: e.containerInfo,
                pendingChildren: null,
                implementation: e.implementation
            }, t;
        }
        var Oc = new WeakMap;
        function Rt(e, t) {
            if (typeof e == "object" && e !== null) {
                var n = Oc.get(e);
                return n !== void 0 ? n : (t = {
                    value: e,
                    source: t,
                    stack: jo(t)
                }, Oc.set(e, t), t);
            }
            return {
                value: e,
                source: t,
                stack: jo(t)
            };
        }
        var Cl = [], Rl = 0, _i = null, da = 0, wt = [], Et = 0, dn = null, Vt = 1, Lt = "";
        function Kt(e, t) {
            Cl[Rl++] = da, Cl[Rl++] = _i, _i = e, da = t;
        }
        function Tc(e, t, n) {
            wt[Et++] = Vt, wt[Et++] = Lt, wt[Et++] = dn, dn = e;
            var a = Vt;
            e = Lt;
            var s = 32 - dt(a) - 1;
            a &= ~(1 << s), n += 1;
            var o = 32 - dt(t) + s;
            if (30 < o) {
                var f = s - s % 5;
                o = (a & (1 << f) - 1).toString(32), a >>= f, s -= f, Vt = 1 << 32 - dt(t) + s | n << s | a, Lt = o + e;
            } else Vt = 1 << o | n << s | a, Lt = e;
        }
        function cs(e) {
            e.return !== null && (Kt(e, 1), Tc(e, 1, 0));
        }
        function fs(e) {
            for(; e === _i;)_i = Cl[--Rl], Cl[Rl] = null, da = Cl[--Rl], Cl[Rl] = null;
            for(; e === dn;)dn = wt[--Et], wt[Et] = null, Lt = wt[--Et], wt[Et] = null, Vt = wt[--Et], wt[Et] = null;
        }
        function Nc(e, t) {
            wt[Et++] = Vt, wt[Et++] = Lt, wt[Et++] = dn, Vt = t.id, Lt = t.overflow, dn = e;
        }
        var Fe = null, je = null, de = !1, hn = null, Mt = !1, ds = Error(r(519));
        function gn(e) {
            var t = Error(r(418, 1 < arguments.length && arguments[1] !== void 0 && arguments[1] ? "text" : "HTML", ""));
            throw ha(Rt(t, e)), ds;
        }
        function Ac(e) {
            var t = e.stateNode, n = e.type, a = e.memoizedProps;
            switch(t[Xe] = e, t[nt] = a, n){
                case "dialog":
                    re("cancel", t), re("close", t);
                    break;
                case "iframe":
                case "object":
                case "embed":
                    re("load", t);
                    break;
                case "video":
                case "audio":
                    for(n = 0; n < Ha.length; n++)re(Ha[n], t);
                    break;
                case "source":
                    re("error", t);
                    break;
                case "img":
                case "image":
                case "link":
                    re("error", t), re("load", t);
                    break;
                case "details":
                    re("toggle", t);
                    break;
                case "input":
                    re("invalid", t), Fo(t, a.value, a.defaultValue, a.checked, a.defaultChecked, a.type, a.name, !0);
                    break;
                case "select":
                    re("invalid", t);
                    break;
                case "textarea":
                    re("invalid", t), Ko(t, a.value, a.defaultValue, a.children);
            }
            n = a.children, typeof n != "string" && typeof n != "number" && typeof n != "bigint" || t.textContent === "" + n || a.suppressHydrationWarning === !0 || kd(t.textContent, n) ? (a.popover != null && (re("beforetoggle", t), re("toggle", t)), a.onScroll != null && re("scroll", t), a.onScrollEnd != null && re("scrollend", t), a.onClick != null && (t.onclick = Xt), t = !0) : t = !1, t || gn(e, !0);
        }
        function zc(e) {
            for(Fe = e.return; Fe;)switch(Fe.tag){
                case 5:
                case 31:
                case 13:
                    Mt = !1;
                    return;
                case 27:
                case 3:
                    Mt = !0;
                    return;
                default:
                    Fe = Fe.return;
            }
        }
        function wl(e) {
            if (e !== Fe) return !1;
            if (!de) return zc(e), de = !0, !1;
            var t = e.tag, n;
            if ((n = t !== 3 && t !== 27) && ((n = t === 5) && (n = e.type, n = !(n !== "form" && n !== "button") || Tr(e.type, e.memoizedProps)), n = !n), n && je && gn(e), zc(e), t === 13) {
                if (e = e.memoizedState, e = e !== null ? e.dehydrated : null, !e) throw Error(r(317));
                je = ih(e);
            } else if (t === 31) {
                if (e = e.memoizedState, e = e !== null ? e.dehydrated : null, !e) throw Error(r(317));
                je = ih(e);
            } else t === 27 ? (t = je, jn(e.type) ? (e = Hr, Hr = null, je = e) : je = t) : je = Fe ? Ot(e.stateNode.nextSibling) : null;
            return !0;
        }
        function $n() {
            je = Fe = null, de = !1;
        }
        function hs() {
            var e = hn;
            return e !== null && (st === null ? st = e : st.push.apply(st, e), hn = null), e;
        }
        function ha(e) {
            hn === null ? hn = [
                e
            ] : hn.push(e);
        }
        var gs = x(null), Jn = null, $t = null;
        function mn(e, t, n) {
            L(gs, t._currentValue), t._currentValue = n;
        }
        function Jt(e) {
            e._currentValue = gs.current, H(gs);
        }
        function ms(e, t, n) {
            for(; e !== null;){
                var a = e.alternate;
                if ((e.childLanes & t) !== t ? (e.childLanes |= t, a !== null && (a.childLanes |= t)) : a !== null && (a.childLanes & t) !== t && (a.childLanes |= t), e === n) break;
                e = e.return;
            }
        }
        function ys(e, t, n, a) {
            var s = e.child;
            for(s !== null && (s.return = e); s !== null;){
                var o = s.dependencies;
                if (o !== null) {
                    var f = s.child;
                    o = o.firstContext;
                    e: for(; o !== null;){
                        var g = o;
                        o = s;
                        for(var S = 0; S < t.length; S++)if (g.context === t[S]) {
                            o.lanes |= n, g = o.alternate, g !== null && (g.lanes |= n), ms(o.return, n, e), a || (f = null);
                            break e;
                        }
                        o = g.next;
                    }
                } else if (s.tag === 18) {
                    if (f = s.return, f === null) throw Error(r(341));
                    f.lanes |= n, o = f.alternate, o !== null && (o.lanes |= n), ms(f, n, e), f = null;
                } else f = s.child;
                if (f !== null) f.return = s;
                else for(f = s; f !== null;){
                    if (f === e) {
                        f = null;
                        break;
                    }
                    if (s = f.sibling, s !== null) {
                        s.return = f.return, f = s;
                        break;
                    }
                    f = f.return;
                }
                s = f;
            }
        }
        function El(e, t, n, a) {
            e = null;
            for(var s = t, o = !1; s !== null;){
                if (!o) {
                    if ((s.flags & 524288) !== 0) o = !0;
                    else if ((s.flags & 262144) !== 0) break;
                }
                if (s.tag === 10) {
                    var f = s.alternate;
                    if (f === null) throw Error(r(387));
                    if (f = f.memoizedProps, f !== null) {
                        var g = s.type;
                        ht(s.pendingProps.value, f.value) || (e !== null ? e.push(g) : e = [
                            g
                        ]);
                    }
                } else if (s === ve.current) {
                    if (f = s.alternate, f === null) throw Error(r(387));
                    f.memoizedState.memoizedState !== s.memoizedState.memoizedState && (e !== null ? e.push(Ga) : e = [
                        Ga
                    ]);
                }
                s = s.return;
            }
            e !== null && ys(t, e, n, a), t.flags |= 262144;
        }
        function Ci(e) {
            for(e = e.firstContext; e !== null;){
                if (!ht(e.context._currentValue, e.memoizedValue)) return !0;
                e = e.next;
            }
            return !1;
        }
        function kn(e) {
            Jn = e, $t = null, e = e.dependencies, e !== null && (e.firstContext = null);
        }
        function Ze(e) {
            return Dc(Jn, e);
        }
        function Ri(e, t) {
            return Jn === null && kn(e), Dc(e, t);
        }
        function Dc(e, t) {
            var n = t._currentValue;
            if (t = {
                context: t,
                memoizedValue: n,
                next: null
            }, $t === null) {
                if (e === null) throw Error(r(308));
                $t = t, e.dependencies = {
                    lanes: 0,
                    firstContext: t
                }, e.flags |= 524288;
            } else $t = $t.next = t;
            return n;
        }
        var Qm = typeof AbortController < "u" ? AbortController : function() {
            var e = [], t = this.signal = {
                aborted: !1,
                addEventListener: function(n, a) {
                    e.push(a);
                }
            };
            this.abort = function() {
                t.aborted = !0, e.forEach(function(n) {
                    return n();
                });
            };
        }, Ym = l.unstable_scheduleCallback, Xm = l.unstable_NormalPriority, qe = {
            $$typeof: Z,
            Consumer: null,
            Provider: null,
            _currentValue: null,
            _currentValue2: null,
            _threadCount: 0
        };
        function vs() {
            return {
                controller: new Qm,
                data: new Map,
                refCount: 0
            };
        }
        function ga(e) {
            e.refCount--, e.refCount === 0 && Ym(Xm, function() {
                e.controller.abort();
            });
        }
        var ma = null, ps = 0, Ml = 0, jl = null;
        function Fm(e, t) {
            if (ma === null) {
                var n = ma = [];
                ps = 0, Ml = xr(), jl = {
                    status: "pending",
                    value: void 0,
                    then: function(a) {
                        n.push(a);
                    }
                };
            }
            return ps++, t.then(Hc, Hc), t;
        }
        function Hc() {
            if (--ps === 0 && ma !== null) {
                jl !== null && (jl.status = "fulfilled");
                var e = ma;
                ma = null, Ml = 0, jl = null;
                for(var t = 0; t < e.length; t++)(0, e[t])();
            }
        }
        function Zm(e, t) {
            var n = [], a = {
                status: "pending",
                value: null,
                reason: null,
                then: function(s) {
                    n.push(s);
                }
            };
            return e.then(function() {
                a.status = "fulfilled", a.value = t;
                for(var s = 0; s < n.length; s++)(0, n[s])(t);
            }, function(s) {
                for(a.status = "rejected", a.reason = s, s = 0; s < n.length; s++)(0, n[s])(void 0);
            }), a;
        }
        var qc = A.S;
        A.S = function(e, t) {
            bd = ct(), typeof t == "object" && t !== null && typeof t.then == "function" && Fm(e, t), qc !== null && qc(e, t);
        };
        var Wn = x(null);
        function Ss() {
            var e = Wn.current;
            return e !== null ? e : Ee.pooledCache;
        }
        function wi(e, t) {
            t === null ? L(Wn, Wn.current) : L(Wn, t.pool);
        }
        function Uc() {
            var e = Ss();
            return e === null ? null : {
                parent: qe._currentValue,
                pool: e
            };
        }
        var Ol = Error(r(460)), bs = Error(r(474)), Ei = Error(r(542)), Mi = {
            then: function() {}
        };
        function Vc(e) {
            return e = e.status, e === "fulfilled" || e === "rejected";
        }
        function Lc(e, t, n) {
            switch(n = e[n], n === void 0 ? e.push(t) : n !== t && (t.then(Xt, Xt), t = n), t.status){
                case "fulfilled":
                    return t.value;
                case "rejected":
                    throw e = t.reason, Bc(e), e;
                default:
                    if (typeof t.status == "string") t.then(Xt, Xt);
                    else {
                        if (e = Ee, e !== null && 100 < e.shellSuspendCounter) throw Error(r(482));
                        e = t, e.status = "pending", e.then(function(a) {
                            if (t.status === "pending") {
                                var s = t;
                                s.status = "fulfilled", s.value = a;
                            }
                        }, function(a) {
                            if (t.status === "pending") {
                                var s = t;
                                s.status = "rejected", s.reason = a;
                            }
                        });
                    }
                    switch(t.status){
                        case "fulfilled":
                            return t.value;
                        case "rejected":
                            throw e = t.reason, Bc(e), e;
                    }
                    throw In = t, Ol;
            }
        }
        function Pn(e) {
            try {
                var t = e._init;
                return t(e._payload);
            } catch (n) {
                throw n !== null && typeof n == "object" && typeof n.then == "function" ? (In = n, Ol) : n;
            }
        }
        var In = null;
        function Gc() {
            if (In === null) throw Error(r(459));
            var e = In;
            return In = null, e;
        }
        function Bc(e) {
            if (e === Ol || e === Ei) throw Error(r(483));
        }
        var Tl = null, ya = 0;
        function ji(e) {
            var t = ya;
            return ya += 1, Tl === null && (Tl = []), Lc(Tl, e, t);
        }
        function va(e, t) {
            t = t.props.ref, e.ref = t !== void 0 ? t : null;
        }
        function Oi(e, t) {
            throw t.$$typeof === j ? Error(r(525)) : (e = Object.prototype.toString.call(t), Error(r(31, e === "[object Object]" ? "object with keys {" + Object.keys(t).join(", ") + "}" : e)));
        }
        function Qc(e) {
            function t(C, b) {
                if (e) {
                    var R = C.deletions;
                    R === null ? (C.deletions = [
                        b
                    ], C.flags |= 16) : R.push(b);
                }
            }
            function n(C, b) {
                if (!e) return null;
                for(; b !== null;)t(C, b), b = b.sibling;
                return null;
            }
            function a(C) {
                for(var b = new Map; C !== null;)C.key !== null ? b.set(C.key, C) : b.set(C.index, C), C = C.sibling;
                return b;
            }
            function s(C, b) {
                return C = Zt(C, b), C.index = 0, C.sibling = null, C;
            }
            function o(C, b, R) {
                return C.index = R, e ? (R = C.alternate, R !== null ? (R = R.index, R < b ? (C.flags |= 67108866, b) : R) : (C.flags |= 67108866, b)) : (C.flags |= 1048576, b);
            }
            function f(C) {
                return e && C.alternate === null && (C.flags |= 67108866), C;
            }
            function g(C, b, R, z) {
                return b === null || b.tag !== 6 ? (b = rs(R, C.mode, z), b.return = C, b) : (b = s(b, R), b.return = C, b);
            }
            function S(C, b, R, z) {
                var K = R.type;
                return K === q ? N(C, b, R.props.children, z, R.key) : b !== null && (b.elementType === K || typeof K == "object" && K !== null && K.$$typeof === I && Pn(K) === b.type) ? (b = s(b, R.props), va(b, R), b.return = C, b) : (b = xi(R.type, R.key, R.props, null, C.mode, z), va(b, R), b.return = C, b);
            }
            function w(C, b, R, z) {
                return b === null || b.tag !== 4 || b.stateNode.containerInfo !== R.containerInfo || b.stateNode.implementation !== R.implementation ? (b = os(R, C.mode, z), b.return = C, b) : (b = s(b, R.children || []), b.return = C, b);
            }
            function N(C, b, R, z, K) {
                return b === null || b.tag !== 7 ? (b = Kn(R, C.mode, z, K), b.return = C, b) : (b = s(b, R), b.return = C, b);
            }
            function D(C, b, R) {
                if (typeof b == "string" && b !== "" || typeof b == "number" || typeof b == "bigint") return b = rs("" + b, C.mode, R), b.return = C, b;
                if (typeof b == "object" && b !== null) {
                    switch(b.$$typeof){
                        case T:
                            return R = xi(b.type, b.key, b.props, null, C.mode, R), va(R, b), R.return = C, R;
                        case U:
                            return b = os(b, C.mode, R), b.return = C, b;
                        case I:
                            return b = Pn(b), D(C, b, R);
                    }
                    if (At(b) || tt(b)) return b = Kn(b, C.mode, R, null), b.return = C, b;
                    if (typeof b.then == "function") return D(C, ji(b), R);
                    if (b.$$typeof === Z) return D(C, Ri(C, b), R);
                    Oi(C, b);
                }
                return null;
            }
            function E(C, b, R, z) {
                var K = b !== null ? b.key : null;
                if (typeof R == "string" && R !== "" || typeof R == "number" || typeof R == "bigint") return K !== null ? null : g(C, b, "" + R, z);
                if (typeof R == "object" && R !== null) {
                    switch(R.$$typeof){
                        case T:
                            return R.key === K ? S(C, b, R, z) : null;
                        case U:
                            return R.key === K ? w(C, b, R, z) : null;
                        case I:
                            return R = Pn(R), E(C, b, R, z);
                    }
                    if (At(R) || tt(R)) return K !== null ? null : N(C, b, R, z, null);
                    if (typeof R.then == "function") return E(C, b, ji(R), z);
                    if (R.$$typeof === Z) return E(C, b, Ri(C, R), z);
                    Oi(C, R);
                }
                return null;
            }
            function O(C, b, R, z, K) {
                if (typeof z == "string" && z !== "" || typeof z == "number" || typeof z == "bigint") return C = C.get(R) || null, g(b, C, "" + z, K);
                if (typeof z == "object" && z !== null) {
                    switch(z.$$typeof){
                        case T:
                            return C = C.get(z.key === null ? R : z.key) || null, S(b, C, z, K);
                        case U:
                            return C = C.get(z.key === null ? R : z.key) || null, w(b, C, z, K);
                        case I:
                            return z = Pn(z), O(C, b, R, z, K);
                    }
                    if (At(z) || tt(z)) return C = C.get(R) || null, N(b, C, z, K, null);
                    if (typeof z.then == "function") return O(C, b, R, ji(z), K);
                    if (z.$$typeof === Z) return O(C, b, R, Ri(b, z), K);
                    Oi(b, z);
                }
                return null;
            }
            function Q(C, b, R, z) {
                for(var K = null, ge = null, X = b, le = b = 0, fe = null; X !== null && le < R.length; le++){
                    X.index > le ? (fe = X, X = null) : fe = X.sibling;
                    var me = E(C, X, R[le], z);
                    if (me === null) {
                        X === null && (X = fe);
                        break;
                    }
                    e && X && me.alternate === null && t(C, X), b = o(me, b, le), ge === null ? K = me : ge.sibling = me, ge = me, X = fe;
                }
                if (le === R.length) return n(C, X), de && Kt(C, le), K;
                if (X === null) {
                    for(; le < R.length; le++)X = D(C, R[le], z), X !== null && (b = o(X, b, le), ge === null ? K = X : ge.sibling = X, ge = X);
                    return de && Kt(C, le), K;
                }
                for(X = a(X); le < R.length; le++)fe = O(X, C, le, R[le], z), fe !== null && (e && fe.alternate !== null && X.delete(fe.key === null ? le : fe.key), b = o(fe, b, le), ge === null ? K = fe : ge.sibling = fe, ge = fe);
                return e && X.forEach(function(zn) {
                    return t(C, zn);
                }), de && Kt(C, le), K;
            }
            function J(C, b, R, z) {
                if (R == null) throw Error(r(151));
                for(var K = null, ge = null, X = b, le = b = 0, fe = null, me = R.next(); X !== null && !me.done; le++, me = R.next()){
                    X.index > le ? (fe = X, X = null) : fe = X.sibling;
                    var zn = E(C, X, me.value, z);
                    if (zn === null) {
                        X === null && (X = fe);
                        break;
                    }
                    e && X && zn.alternate === null && t(C, X), b = o(zn, b, le), ge === null ? K = zn : ge.sibling = zn, ge = zn, X = fe;
                }
                if (me.done) return n(C, X), de && Kt(C, le), K;
                if (X === null) {
                    for(; !me.done; le++, me = R.next())me = D(C, me.value, z), me !== null && (b = o(me, b, le), ge === null ? K = me : ge.sibling = me, ge = me);
                    return de && Kt(C, le), K;
                }
                for(X = a(X); !me.done; le++, me = R.next())me = O(X, C, le, me.value, z), me !== null && (e && me.alternate !== null && X.delete(me.key === null ? le : me.key), b = o(me, b, le), ge === null ? K = me : ge.sibling = me, ge = me);
                return e && X.forEach(function(ly) {
                    return t(C, ly);
                }), de && Kt(C, le), K;
            }
            function Re(C, b, R, z) {
                if (typeof R == "object" && R !== null && R.type === q && R.key === null && (R = R.props.children), typeof R == "object" && R !== null) {
                    switch(R.$$typeof){
                        case T:
                            e: {
                                for(var K = R.key; b !== null;){
                                    if (b.key === K) {
                                        if (K = R.type, K === q) {
                                            if (b.tag === 7) {
                                                n(C, b.sibling), z = s(b, R.props.children), z.return = C, C = z;
                                                break e;
                                            }
                                        } else if (b.elementType === K || typeof K == "object" && K !== null && K.$$typeof === I && Pn(K) === b.type) {
                                            n(C, b.sibling), z = s(b, R.props), va(z, R), z.return = C, C = z;
                                            break e;
                                        }
                                        n(C, b);
                                        break;
                                    } else t(C, b);
                                    b = b.sibling;
                                }
                                R.type === q ? (z = Kn(R.props.children, C.mode, z, R.key), z.return = C, C = z) : (z = xi(R.type, R.key, R.props, null, C.mode, z), va(z, R), z.return = C, C = z);
                            }
                            return f(C);
                        case U:
                            e: {
                                for(K = R.key; b !== null;){
                                    if (b.key === K) if (b.tag === 4 && b.stateNode.containerInfo === R.containerInfo && b.stateNode.implementation === R.implementation) {
                                        n(C, b.sibling), z = s(b, R.children || []), z.return = C, C = z;
                                        break e;
                                    } else {
                                        n(C, b);
                                        break;
                                    }
                                    else t(C, b);
                                    b = b.sibling;
                                }
                                z = os(R, C.mode, z), z.return = C, C = z;
                            }
                            return f(C);
                        case I:
                            return R = Pn(R), Re(C, b, R, z);
                    }
                    if (At(R)) return Q(C, b, R, z);
                    if (tt(R)) {
                        if (K = tt(R), typeof K != "function") throw Error(r(150));
                        return R = K.call(R), J(C, b, R, z);
                    }
                    if (typeof R.then == "function") return Re(C, b, ji(R), z);
                    if (R.$$typeof === Z) return Re(C, b, Ri(C, R), z);
                    Oi(C, R);
                }
                return typeof R == "string" && R !== "" || typeof R == "number" || typeof R == "bigint" ? (R = "" + R, b !== null && b.tag === 6 ? (n(C, b.sibling), z = s(b, R), z.return = C, C = z) : (n(C, b), z = rs(R, C.mode, z), z.return = C, C = z), f(C)) : n(C, b);
            }
            return function(C, b, R, z) {
                try {
                    ya = 0;
                    var K = Re(C, b, R, z);
                    return Tl = null, K;
                } catch (X) {
                    if (X === Ol || X === Ei) throw X;
                    var ge = gt(29, X, null, C.mode);
                    return ge.lanes = z, ge.return = C, ge;
                }
            };
        }
        var el = Qc(!0), Yc = Qc(!1), yn = !1;
        function xs(e) {
            e.updateQueue = {
                baseState: e.memoizedState,
                firstBaseUpdate: null,
                lastBaseUpdate: null,
                shared: {
                    pending: null,
                    lanes: 0,
                    hiddenCallbacks: null
                },
                callbacks: null
            };
        }
        function _s(e, t) {
            e = e.updateQueue, t.updateQueue === e && (t.updateQueue = {
                baseState: e.baseState,
                firstBaseUpdate: e.firstBaseUpdate,
                lastBaseUpdate: e.lastBaseUpdate,
                shared: e.shared,
                callbacks: null
            });
        }
        function vn(e) {
            return {
                lane: e,
                tag: 0,
                payload: null,
                callback: null,
                next: null
            };
        }
        function pn(e, t, n) {
            var a = e.updateQueue;
            if (a === null) return null;
            if (a = a.shared, (ye & 2) !== 0) {
                var s = a.pending;
                return s === null ? t.next = t : (t.next = s.next, s.next = t), a.pending = t, t = bi(e), Ec(e, null, n), t;
            }
            return Si(e, a, t, n), bi(e);
        }
        function pa(e, t, n) {
            if (t = t.updateQueue, t !== null && (t = t.shared, (n & 4194048) !== 0)) {
                var a = t.lanes;
                a &= e.pendingLanes, n |= a, t.lanes = n, Do(e, n);
            }
        }
        function Cs(e, t) {
            var n = e.updateQueue, a = e.alternate;
            if (a !== null && (a = a.updateQueue, n === a)) {
                var s = null, o = null;
                if (n = n.firstBaseUpdate, n !== null) {
                    do {
                        var f = {
                            lane: n.lane,
                            tag: n.tag,
                            payload: n.payload,
                            callback: null,
                            next: null
                        };
                        o === null ? s = o = f : o = o.next = f, n = n.next;
                    }while (n !== null);
                    o === null ? s = o = t : o = o.next = t;
                } else s = o = t;
                n = {
                    baseState: a.baseState,
                    firstBaseUpdate: s,
                    lastBaseUpdate: o,
                    shared: a.shared,
                    callbacks: a.callbacks
                }, e.updateQueue = n;
                return;
            }
            e = n.lastBaseUpdate, e === null ? n.firstBaseUpdate = t : e.next = t, n.lastBaseUpdate = t;
        }
        var Rs = !1;
        function Sa() {
            if (Rs) {
                var e = jl;
                if (e !== null) throw e;
            }
        }
        function ba(e, t, n, a) {
            Rs = !1;
            var s = e.updateQueue;
            yn = !1;
            var o = s.firstBaseUpdate, f = s.lastBaseUpdate, g = s.shared.pending;
            if (g !== null) {
                s.shared.pending = null;
                var S = g, w = S.next;
                S.next = null, f === null ? o = w : f.next = w, f = S;
                var N = e.alternate;
                N !== null && (N = N.updateQueue, g = N.lastBaseUpdate, g !== f && (g === null ? N.firstBaseUpdate = w : g.next = w, N.lastBaseUpdate = S));
            }
            if (o !== null) {
                var D = s.baseState;
                f = 0, N = w = S = null, g = o;
                do {
                    var E = g.lane & -536870913, O = E !== g.lane;
                    if (O ? (ce & E) === E : (a & E) === E) {
                        E !== 0 && E === Ml && (Rs = !0), N !== null && (N = N.next = {
                            lane: 0,
                            tag: g.tag,
                            payload: g.payload,
                            callback: null,
                            next: null
                        });
                        e: {
                            var Q = e, J = g;
                            E = t;
                            var Re = n;
                            switch(J.tag){
                                case 1:
                                    if (Q = J.payload, typeof Q == "function") {
                                        D = Q.call(Re, D, E);
                                        break e;
                                    }
                                    D = Q;
                                    break e;
                                case 3:
                                    Q.flags = Q.flags & -65537 | 128;
                                case 0:
                                    if (Q = J.payload, E = typeof Q == "function" ? Q.call(Re, D, E) : Q, E == null) break e;
                                    D = M({}, D, E);
                                    break e;
                                case 2:
                                    yn = !0;
                            }
                        }
                        E = g.callback, E !== null && (e.flags |= 64, O && (e.flags |= 8192), O = s.callbacks, O === null ? s.callbacks = [
                            E
                        ] : O.push(E));
                    } else O = {
                        lane: E,
                        tag: g.tag,
                        payload: g.payload,
                        callback: g.callback,
                        next: null
                    }, N === null ? (w = N = O, S = D) : N = N.next = O, f |= E;
                    if (g = g.next, g === null) {
                        if (g = s.shared.pending, g === null) break;
                        O = g, g = O.next, O.next = null, s.lastBaseUpdate = O, s.shared.pending = null;
                    }
                }while (!0);
                N === null && (S = D), s.baseState = S, s.firstBaseUpdate = w, s.lastBaseUpdate = N, o === null && (s.shared.lanes = 0), Cn |= f, e.lanes = f, e.memoizedState = D;
            }
        }
        function Xc(e, t) {
            if (typeof e != "function") throw Error(r(191, e));
            e.call(t);
        }
        function Fc(e, t) {
            var n = e.callbacks;
            if (n !== null) for(e.callbacks = null, e = 0; e < n.length; e++)Xc(n[e], t);
        }
        var Nl = x(null), Ti = x(0);
        function Zc(e, t) {
            e = an, L(Ti, e), L(Nl, t), an = e | t.baseLanes;
        }
        function ws() {
            L(Ti, an), L(Nl, Nl.current);
        }
        function Es() {
            an = Ti.current, H(Nl), H(Ti);
        }
        var mt = x(null), jt = null;
        function Sn(e) {
            var t = e.alternate;
            L(De, De.current & 1), L(mt, e), jt === null && (t === null || Nl.current !== null || t.memoizedState !== null) && (jt = e);
        }
        function Ms(e) {
            L(De, De.current), L(mt, e), jt === null && (jt = e);
        }
        function Kc(e) {
            e.tag === 22 ? (L(De, De.current), L(mt, e), jt === null && (jt = e)) : bn();
        }
        function bn() {
            L(De, De.current), L(mt, mt.current);
        }
        function yt(e) {
            H(mt), jt === e && (jt = null), H(De);
        }
        var De = x(0);
        function Ni(e) {
            for(var t = e; t !== null;){
                if (t.tag === 13) {
                    var n = t.memoizedState;
                    if (n !== null && (n = n.dehydrated, n === null || zr(n) || Dr(n))) return t;
                } else if (t.tag === 19 && (t.memoizedProps.revealOrder === "forwards" || t.memoizedProps.revealOrder === "backwards" || t.memoizedProps.revealOrder === "unstable_legacy-backwards" || t.memoizedProps.revealOrder === "together")) {
                    if ((t.flags & 128) !== 0) return t;
                } else if (t.child !== null) {
                    t.child.return = t, t = t.child;
                    continue;
                }
                if (t === e) break;
                for(; t.sibling === null;){
                    if (t.return === null || t.return === e) return null;
                    t = t.return;
                }
                t.sibling.return = t.return, t = t.sibling;
            }
            return null;
        }
        var kt = 0, ne = null, _e = null, Ue = null, Ai = !1, Al = !1, tl = !1, zi = 0, xa = 0, zl = null, Km = 0;
        function Ne() {
            throw Error(r(321));
        }
        function js(e, t) {
            if (t === null) return !1;
            for(var n = 0; n < t.length && n < e.length; n++)if (!ht(e[n], t[n])) return !1;
            return !0;
        }
        function Os(e, t, n, a, s, o) {
            return kt = o, ne = t, t.memoizedState = null, t.updateQueue = null, t.lanes = 0, A.H = e === null || e.memoizedState === null ? Nf : Xs, tl = !1, o = n(a, s), tl = !1, Al && (o = Jc(t, n, a, s)), $c(e), o;
        }
        function $c(e) {
            A.H = Ra;
            var t = _e !== null && _e.next !== null;
            if (kt = 0, Ue = _e = ne = null, Ai = !1, xa = 0, zl = null, t) throw Error(r(300));
            e === null || Ve || (e = e.dependencies, e !== null && Ci(e) && (Ve = !0));
        }
        function Jc(e, t, n, a) {
            ne = e;
            var s = 0;
            do {
                if (Al && (zl = null), xa = 0, Al = !1, 25 <= s) throw Error(r(301));
                if (s += 1, Ue = _e = null, e.updateQueue != null) {
                    var o = e.updateQueue;
                    o.lastEffect = null, o.events = null, o.stores = null, o.memoCache != null && (o.memoCache.index = 0);
                }
                A.H = Af, o = t(n, a);
            }while (Al);
            return o;
        }
        function $m() {
            var e = A.H, t = e.useState()[0];
            return t = typeof t.then == "function" ? _a(t) : t, e = e.useState()[0], (_e !== null ? _e.memoizedState : null) !== e && (ne.flags |= 1024), t;
        }
        function Ts() {
            var e = zi !== 0;
            return zi = 0, e;
        }
        function Ns(e, t, n) {
            t.updateQueue = e.updateQueue, t.flags &= -2053, e.lanes &= ~n;
        }
        function As(e) {
            if (Ai) {
                for(e = e.memoizedState; e !== null;){
                    var t = e.queue;
                    t !== null && (t.pending = null), e = e.next;
                }
                Ai = !1;
            }
            kt = 0, Ue = _e = ne = null, Al = !1, xa = zi = 0, zl = null;
        }
        function et() {
            var e = {
                memoizedState: null,
                baseState: null,
                baseQueue: null,
                queue: null,
                next: null
            };
            return Ue === null ? ne.memoizedState = Ue = e : Ue = Ue.next = e, Ue;
        }
        function He() {
            if (_e === null) {
                var e = ne.alternate;
                e = e !== null ? e.memoizedState : null;
            } else e = _e.next;
            var t = Ue === null ? ne.memoizedState : Ue.next;
            if (t !== null) Ue = t, _e = e;
            else {
                if (e === null) throw ne.alternate === null ? Error(r(467)) : Error(r(310));
                _e = e, e = {
                    memoizedState: _e.memoizedState,
                    baseState: _e.baseState,
                    baseQueue: _e.baseQueue,
                    queue: _e.queue,
                    next: null
                }, Ue === null ? ne.memoizedState = Ue = e : Ue = Ue.next = e;
            }
            return Ue;
        }
        function Di() {
            return {
                lastEffect: null,
                events: null,
                stores: null,
                memoCache: null
            };
        }
        function _a(e) {
            var t = xa;
            return xa += 1, zl === null && (zl = []), e = Lc(zl, e, t), t = ne, (Ue === null ? t.memoizedState : Ue.next) === null && (t = t.alternate, A.H = t === null || t.memoizedState === null ? Nf : Xs), e;
        }
        function Hi(e) {
            if (e !== null && typeof e == "object") {
                if (typeof e.then == "function") return _a(e);
                if (e.$$typeof === Z) return Ze(e);
            }
            throw Error(r(438, String(e)));
        }
        function zs(e) {
            var t = null, n = ne.updateQueue;
            if (n !== null && (t = n.memoCache), t == null) {
                var a = ne.alternate;
                a !== null && (a = a.updateQueue, a !== null && (a = a.memoCache, a != null && (t = {
                    data: a.data.map(function(s) {
                        return s.slice();
                    }),
                    index: 0
                })));
            }
            if (t == null && (t = {
                data: [],
                index: 0
            }), n === null && (n = Di(), ne.updateQueue = n), n.memoCache = t, n = t.data[t.index], n === void 0) for(n = t.data[t.index] = Array(e), a = 0; a < e; a++)n[a] = Pe;
            return t.index++, n;
        }
        function Wt(e, t) {
            return typeof t == "function" ? t(e) : t;
        }
        function qi(e) {
            var t = He();
            return Ds(t, _e, e);
        }
        function Ds(e, t, n) {
            var a = e.queue;
            if (a === null) throw Error(r(311));
            a.lastRenderedReducer = n;
            var s = e.baseQueue, o = a.pending;
            if (o !== null) {
                if (s !== null) {
                    var f = s.next;
                    s.next = o.next, o.next = f;
                }
                t.baseQueue = s = o, a.pending = null;
            }
            if (o = e.baseState, s === null) e.memoizedState = o;
            else {
                t = s.next;
                var g = f = null, S = null, w = t, N = !1;
                do {
                    var D = w.lane & -536870913;
                    if (D !== w.lane ? (ce & D) === D : (kt & D) === D) {
                        var E = w.revertLane;
                        if (E === 0) S !== null && (S = S.next = {
                            lane: 0,
                            revertLane: 0,
                            gesture: null,
                            action: w.action,
                            hasEagerState: w.hasEagerState,
                            eagerState: w.eagerState,
                            next: null
                        }), D === Ml && (N = !0);
                        else if ((kt & E) === E) {
                            w = w.next, E === Ml && (N = !0);
                            continue;
                        } else D = {
                            lane: 0,
                            revertLane: w.revertLane,
                            gesture: null,
                            action: w.action,
                            hasEagerState: w.hasEagerState,
                            eagerState: w.eagerState,
                            next: null
                        }, S === null ? (g = S = D, f = o) : S = S.next = D, ne.lanes |= E, Cn |= E;
                        D = w.action, tl && n(o, D), o = w.hasEagerState ? w.eagerState : n(o, D);
                    } else E = {
                        lane: D,
                        revertLane: w.revertLane,
                        gesture: w.gesture,
                        action: w.action,
                        hasEagerState: w.hasEagerState,
                        eagerState: w.eagerState,
                        next: null
                    }, S === null ? (g = S = E, f = o) : S = S.next = E, ne.lanes |= D, Cn |= D;
                    w = w.next;
                }while (w !== null && w !== t);
                if (S === null ? f = o : S.next = g, !ht(o, e.memoizedState) && (Ve = !0, N && (n = jl, n !== null))) throw n;
                e.memoizedState = o, e.baseState = f, e.baseQueue = S, a.lastRenderedState = o;
            }
            return s === null && (a.lanes = 0), [
                e.memoizedState,
                a.dispatch
            ];
        }
        function Hs(e) {
            var t = He(), n = t.queue;
            if (n === null) throw Error(r(311));
            n.lastRenderedReducer = e;
            var a = n.dispatch, s = n.pending, o = t.memoizedState;
            if (s !== null) {
                n.pending = null;
                var f = s = s.next;
                do o = e(o, f.action), f = f.next;
                while (f !== s);
                ht(o, t.memoizedState) || (Ve = !0), t.memoizedState = o, t.baseQueue === null && (t.baseState = o), n.lastRenderedState = o;
            }
            return [
                o,
                a
            ];
        }
        function kc(e, t, n) {
            var a = ne, s = He(), o = de;
            if (o) {
                if (n === void 0) throw Error(r(407));
                n = n();
            } else n = t();
            var f = !ht((_e || s).memoizedState, n);
            if (f && (s.memoizedState = n, Ve = !0), s = s.queue, Vs(Ic.bind(null, a, s, e), [
                e
            ]), s.getSnapshot !== t || f || Ue !== null && Ue.memoizedState.tag & 1) {
                if (a.flags |= 2048, Dl(9, {
                    destroy: void 0
                }, Pc.bind(null, a, s, n, t), null), Ee === null) throw Error(r(349));
                o || (kt & 127) !== 0 || Wc(a, t, n);
            }
            return n;
        }
        function Wc(e, t, n) {
            e.flags |= 16384, e = {
                getSnapshot: t,
                value: n
            }, t = ne.updateQueue, t === null ? (t = Di(), ne.updateQueue = t, t.stores = [
                e
            ]) : (n = t.stores, n === null ? t.stores = [
                e
            ] : n.push(e));
        }
        function Pc(e, t, n, a) {
            t.value = n, t.getSnapshot = a, ef(t) && tf(e);
        }
        function Ic(e, t, n) {
            return n(function() {
                ef(t) && tf(e);
            });
        }
        function ef(e) {
            var t = e.getSnapshot;
            e = e.value;
            try {
                var n = t();
                return !ht(e, n);
            } catch  {
                return !0;
            }
        }
        function tf(e) {
            var t = Zn(e, 2);
            t !== null && rt(t, e, 2);
        }
        function qs(e) {
            var t = et();
            if (typeof e == "function") {
                var n = e;
                if (e = n(), tl) {
                    on(!0);
                    try {
                        n();
                    } finally{
                        on(!1);
                    }
                }
            }
            return t.memoizedState = t.baseState = e, t.queue = {
                pending: null,
                lanes: 0,
                dispatch: null,
                lastRenderedReducer: Wt,
                lastRenderedState: e
            }, t;
        }
        function nf(e, t, n, a) {
            return e.baseState = n, Ds(e, _e, typeof a == "function" ? a : Wt);
        }
        function Jm(e, t, n, a, s) {
            if (Li(e)) throw Error(r(485));
            if (e = t.action, e !== null) {
                var o = {
                    payload: s,
                    action: e,
                    next: null,
                    isTransition: !0,
                    status: "pending",
                    value: null,
                    reason: null,
                    listeners: [],
                    then: function(f) {
                        o.listeners.push(f);
                    }
                };
                A.T !== null ? n(!0) : o.isTransition = !1, a(o), n = t.pending, n === null ? (o.next = t.pending = o, lf(t, o)) : (o.next = n.next, t.pending = n.next = o);
            }
        }
        function lf(e, t) {
            var n = t.action, a = t.payload, s = e.state;
            if (t.isTransition) {
                var o = A.T, f = {};
                A.T = f;
                try {
                    var g = n(s, a), S = A.S;
                    S !== null && S(f, g), af(e, t, g);
                } catch (w) {
                    Us(e, t, w);
                } finally{
                    o !== null && f.types !== null && (o.types = f.types), A.T = o;
                }
            } else try {
                o = n(s, a), af(e, t, o);
            } catch (w) {
                Us(e, t, w);
            }
        }
        function af(e, t, n) {
            n !== null && typeof n == "object" && typeof n.then == "function" ? n.then(function(a) {
                uf(e, t, a);
            }, function(a) {
                return Us(e, t, a);
            }) : uf(e, t, n);
        }
        function uf(e, t, n) {
            t.status = "fulfilled", t.value = n, sf(t), e.state = n, t = e.pending, t !== null && (n = t.next, n === t ? e.pending = null : (n = n.next, t.next = n, lf(e, n)));
        }
        function Us(e, t, n) {
            var a = e.pending;
            if (e.pending = null, a !== null) {
                a = a.next;
                do t.status = "rejected", t.reason = n, sf(t), t = t.next;
                while (t !== a);
            }
            e.action = null;
        }
        function sf(e) {
            e = e.listeners;
            for(var t = 0; t < e.length; t++)(0, e[t])();
        }
        function rf(e, t) {
            return t;
        }
        function of(e, t) {
            if (de) {
                var n = Ee.formState;
                if (n !== null) {
                    e: {
                        var a = ne;
                        if (de) {
                            if (je) {
                                t: {
                                    for(var s = je, o = Mt; s.nodeType !== 8;){
                                        if (!o) {
                                            s = null;
                                            break t;
                                        }
                                        if (s = Ot(s.nextSibling), s === null) {
                                            s = null;
                                            break t;
                                        }
                                    }
                                    o = s.data, s = o === "F!" || o === "F" ? s : null;
                                }
                                if (s) {
                                    je = Ot(s.nextSibling), a = s.data === "F!";
                                    break e;
                                }
                            }
                            gn(a);
                        }
                        a = !1;
                    }
                    a && (t = n[0]);
                }
            }
            return n = et(), n.memoizedState = n.baseState = t, a = {
                pending: null,
                lanes: 0,
                dispatch: null,
                lastRenderedReducer: rf,
                lastRenderedState: t
            }, n.queue = a, n = jf.bind(null, ne, a), a.dispatch = n, a = qs(!1), o = Ys.bind(null, ne, !1, a.queue), a = et(), s = {
                state: t,
                dispatch: null,
                action: e,
                pending: null
            }, a.queue = s, n = Jm.bind(null, ne, s, o, n), s.dispatch = n, a.memoizedState = e, [
                t,
                n,
                !1
            ];
        }
        function cf(e) {
            var t = He();
            return ff(t, _e, e);
        }
        function ff(e, t, n) {
            if (t = Ds(e, t, rf)[0], e = qi(Wt)[0], typeof t == "object" && t !== null && typeof t.then == "function") try {
                var a = _a(t);
            } catch (f) {
                throw f === Ol ? Ei : f;
            }
            else a = t;
            t = He();
            var s = t.queue, o = s.dispatch;
            return n !== t.memoizedState && (ne.flags |= 2048, Dl(9, {
                destroy: void 0
            }, km.bind(null, s, n), null)), [
                a,
                o,
                e
            ];
        }
        function km(e, t) {
            e.action = t;
        }
        function df(e) {
            var t = He(), n = _e;
            if (n !== null) return ff(t, n, e);
            He(), t = t.memoizedState, n = He();
            var a = n.queue.dispatch;
            return n.memoizedState = e, [
                t,
                a,
                !1
            ];
        }
        function Dl(e, t, n, a) {
            return e = {
                tag: e,
                create: n,
                deps: a,
                inst: t,
                next: null
            }, t = ne.updateQueue, t === null && (t = Di(), ne.updateQueue = t), n = t.lastEffect, n === null ? t.lastEffect = e.next = e : (a = n.next, n.next = e, e.next = a, t.lastEffect = e), e;
        }
        function hf() {
            return He().memoizedState;
        }
        function Ui(e, t, n, a) {
            var s = et();
            ne.flags |= e, s.memoizedState = Dl(1 | t, {
                destroy: void 0
            }, n, a === void 0 ? null : a);
        }
        function Vi(e, t, n, a) {
            var s = He();
            a = a === void 0 ? null : a;
            var o = s.memoizedState.inst;
            _e !== null && a !== null && js(a, _e.memoizedState.deps) ? s.memoizedState = Dl(t, o, n, a) : (ne.flags |= e, s.memoizedState = Dl(1 | t, o, n, a));
        }
        function gf(e, t) {
            Ui(8390656, 8, e, t);
        }
        function Vs(e, t) {
            Vi(2048, 8, e, t);
        }
        function Wm(e) {
            ne.flags |= 4;
            var t = ne.updateQueue;
            if (t === null) t = Di(), ne.updateQueue = t, t.events = [
                e
            ];
            else {
                var n = t.events;
                n === null ? t.events = [
                    e
                ] : n.push(e);
            }
        }
        function mf(e) {
            var t = He().memoizedState;
            return Wm({
                ref: t,
                nextImpl: e
            }), function() {
                if ((ye & 2) !== 0) throw Error(r(440));
                return t.impl.apply(void 0, arguments);
            };
        }
        function yf(e, t) {
            return Vi(4, 2, e, t);
        }
        function vf(e, t) {
            return Vi(4, 4, e, t);
        }
        function pf(e, t) {
            if (typeof t == "function") {
                e = e();
                var n = t(e);
                return function() {
                    typeof n == "function" ? n() : t(null);
                };
            }
            if (t != null) return e = e(), t.current = e, function() {
                t.current = null;
            };
        }
        function Sf(e, t, n) {
            n = n != null ? n.concat([
                e
            ]) : null, Vi(4, 4, pf.bind(null, t, e), n);
        }
        function Ls() {}
        function bf(e, t) {
            var n = He();
            t = t === void 0 ? null : t;
            var a = n.memoizedState;
            return t !== null && js(t, a[1]) ? a[0] : (n.memoizedState = [
                e,
                t
            ], e);
        }
        function xf(e, t) {
            var n = He();
            t = t === void 0 ? null : t;
            var a = n.memoizedState;
            if (t !== null && js(t, a[1])) return a[0];
            if (a = e(), tl) {
                on(!0);
                try {
                    e();
                } finally{
                    on(!1);
                }
            }
            return n.memoizedState = [
                a,
                t
            ], a;
        }
        function Gs(e, t, n) {
            return n === void 0 || (kt & 1073741824) !== 0 && (ce & 261930) === 0 ? e.memoizedState = t : (e.memoizedState = n, e = _d(), ne.lanes |= e, Cn |= e, n);
        }
        function _f(e, t, n, a) {
            return ht(n, t) ? n : Nl.current !== null ? (e = Gs(e, n, a), ht(e, t) || (Ve = !0), e) : (kt & 42) === 0 || (kt & 1073741824) !== 0 && (ce & 261930) === 0 ? (Ve = !0, e.memoizedState = n) : (e = _d(), ne.lanes |= e, Cn |= e, t);
        }
        function Cf(e, t, n, a, s) {
            var o = V.p;
            V.p = o !== 0 && 8 > o ? o : 8;
            var f = A.T, g = {};
            A.T = g, Ys(e, !1, t, n);
            try {
                var S = s(), w = A.S;
                if (w !== null && w(g, S), S !== null && typeof S == "object" && typeof S.then == "function") {
                    var N = Zm(S, a);
                    Ca(e, t, N, St(e));
                } else Ca(e, t, a, St(e));
            } catch (D) {
                Ca(e, t, {
                    then: function() {},
                    status: "rejected",
                    reason: D
                }, St());
            } finally{
                V.p = o, f !== null && g.types !== null && (f.types = g.types), A.T = f;
            }
        }
        function Pm() {}
        function Bs(e, t, n, a) {
            if (e.tag !== 5) throw Error(r(476));
            var s = Rf(e).queue;
            Cf(e, s, t, k, n === null ? Pm : function() {
                return wf(e), n(a);
            });
        }
        function Rf(e) {
            var t = e.memoizedState;
            if (t !== null) return t;
            t = {
                memoizedState: k,
                baseState: k,
                baseQueue: null,
                queue: {
                    pending: null,
                    lanes: 0,
                    dispatch: null,
                    lastRenderedReducer: Wt,
                    lastRenderedState: k
                },
                next: null
            };
            var n = {};
            return t.next = {
                memoizedState: n,
                baseState: n,
                baseQueue: null,
                queue: {
                    pending: null,
                    lanes: 0,
                    dispatch: null,
                    lastRenderedReducer: Wt,
                    lastRenderedState: n
                },
                next: null
            }, e.memoizedState = t, e = e.alternate, e !== null && (e.memoizedState = t), t;
        }
        function wf(e) {
            var t = Rf(e);
            t.next === null && (t = e.alternate.memoizedState), Ca(e, t.next.queue, {}, St());
        }
        function Qs() {
            return Ze(Ga);
        }
        function Ef() {
            return He().memoizedState;
        }
        function Mf() {
            return He().memoizedState;
        }
        function Im(e) {
            for(var t = e.return; t !== null;){
                switch(t.tag){
                    case 24:
                    case 3:
                        var n = St();
                        e = vn(n);
                        var a = pn(t, e, n);
                        a !== null && (rt(a, t, n), pa(a, t, n)), t = {
                            cache: vs()
                        }, e.payload = t;
                        return;
                }
                t = t.return;
            }
        }
        function e0(e, t, n) {
            var a = St();
            n = {
                lane: a,
                revertLane: 0,
                gesture: null,
                action: n,
                hasEagerState: !1,
                eagerState: null,
                next: null
            }, Li(e) ? Of(t, n) : (n = us(e, t, n, a), n !== null && (rt(n, e, a), Tf(n, t, a)));
        }
        function jf(e, t, n) {
            var a = St();
            Ca(e, t, n, a);
        }
        function Ca(e, t, n, a) {
            var s = {
                lane: a,
                revertLane: 0,
                gesture: null,
                action: n,
                hasEagerState: !1,
                eagerState: null,
                next: null
            };
            if (Li(e)) Of(t, s);
            else {
                var o = e.alternate;
                if (e.lanes === 0 && (o === null || o.lanes === 0) && (o = t.lastRenderedReducer, o !== null)) try {
                    var f = t.lastRenderedState, g = o(f, n);
                    if (s.hasEagerState = !0, s.eagerState = g, ht(g, f)) return Si(e, t, s, 0), Ee === null && pi(), !1;
                } catch  {}
                if (n = us(e, t, s, a), n !== null) return rt(n, e, a), Tf(n, t, a), !0;
            }
            return !1;
        }
        function Ys(e, t, n, a) {
            if (a = {
                lane: 2,
                revertLane: xr(),
                gesture: null,
                action: a,
                hasEagerState: !1,
                eagerState: null,
                next: null
            }, Li(e)) {
                if (t) throw Error(r(479));
            } else t = us(e, n, a, 2), t !== null && rt(t, e, 2);
        }
        function Li(e) {
            var t = e.alternate;
            return e === ne || t !== null && t === ne;
        }
        function Of(e, t) {
            Al = Ai = !0;
            var n = e.pending;
            n === null ? t.next = t : (t.next = n.next, n.next = t), e.pending = t;
        }
        function Tf(e, t, n) {
            if ((n & 4194048) !== 0) {
                var a = t.lanes;
                a &= e.pendingLanes, n |= a, t.lanes = n, Do(e, n);
            }
        }
        var Ra = {
            readContext: Ze,
            use: Hi,
            useCallback: Ne,
            useContext: Ne,
            useEffect: Ne,
            useImperativeHandle: Ne,
            useLayoutEffect: Ne,
            useInsertionEffect: Ne,
            useMemo: Ne,
            useReducer: Ne,
            useRef: Ne,
            useState: Ne,
            useDebugValue: Ne,
            useDeferredValue: Ne,
            useTransition: Ne,
            useSyncExternalStore: Ne,
            useId: Ne,
            useHostTransitionStatus: Ne,
            useFormState: Ne,
            useActionState: Ne,
            useOptimistic: Ne,
            useMemoCache: Ne,
            useCacheRefresh: Ne
        };
        Ra.useEffectEvent = Ne;
        var Nf = {
            readContext: Ze,
            use: Hi,
            useCallback: function(e, t) {
                return et().memoizedState = [
                    e,
                    t === void 0 ? null : t
                ], e;
            },
            useContext: Ze,
            useEffect: gf,
            useImperativeHandle: function(e, t, n) {
                n = n != null ? n.concat([
                    e
                ]) : null, Ui(4194308, 4, pf.bind(null, t, e), n);
            },
            useLayoutEffect: function(e, t) {
                return Ui(4194308, 4, e, t);
            },
            useInsertionEffect: function(e, t) {
                Ui(4, 2, e, t);
            },
            useMemo: function(e, t) {
                var n = et();
                t = t === void 0 ? null : t;
                var a = e();
                if (tl) {
                    on(!0);
                    try {
                        e();
                    } finally{
                        on(!1);
                    }
                }
                return n.memoizedState = [
                    a,
                    t
                ], a;
            },
            useReducer: function(e, t, n) {
                var a = et();
                if (n !== void 0) {
                    var s = n(t);
                    if (tl) {
                        on(!0);
                        try {
                            n(t);
                        } finally{
                            on(!1);
                        }
                    }
                } else s = t;
                return a.memoizedState = a.baseState = s, e = {
                    pending: null,
                    lanes: 0,
                    dispatch: null,
                    lastRenderedReducer: e,
                    lastRenderedState: s
                }, a.queue = e, e = e.dispatch = e0.bind(null, ne, e), [
                    a.memoizedState,
                    e
                ];
            },
            useRef: function(e) {
                var t = et();
                return e = {
                    current: e
                }, t.memoizedState = e;
            },
            useState: function(e) {
                e = qs(e);
                var t = e.queue, n = jf.bind(null, ne, t);
                return t.dispatch = n, [
                    e.memoizedState,
                    n
                ];
            },
            useDebugValue: Ls,
            useDeferredValue: function(e, t) {
                var n = et();
                return Gs(n, e, t);
            },
            useTransition: function() {
                var e = qs(!1);
                return e = Cf.bind(null, ne, e.queue, !0, !1), et().memoizedState = e, [
                    !1,
                    e
                ];
            },
            useSyncExternalStore: function(e, t, n) {
                var a = ne, s = et();
                if (de) {
                    if (n === void 0) throw Error(r(407));
                    n = n();
                } else {
                    if (n = t(), Ee === null) throw Error(r(349));
                    (ce & 127) !== 0 || Wc(a, t, n);
                }
                s.memoizedState = n;
                var o = {
                    value: n,
                    getSnapshot: t
                };
                return s.queue = o, gf(Ic.bind(null, a, o, e), [
                    e
                ]), a.flags |= 2048, Dl(9, {
                    destroy: void 0
                }, Pc.bind(null, a, o, n, t), null), n;
            },
            useId: function() {
                var e = et(), t = Ee.identifierPrefix;
                if (de) {
                    var n = Lt, a = Vt;
                    n = (a & ~(1 << 32 - dt(a) - 1)).toString(32) + n, t = "_" + t + "R_" + n, n = zi++, 0 < n && (t += "H" + n.toString(32)), t += "_";
                } else n = Km++, t = "_" + t + "r_" + n.toString(32) + "_";
                return e.memoizedState = t;
            },
            useHostTransitionStatus: Qs,
            useFormState: of,
            useActionState: of,
            useOptimistic: function(e) {
                var t = et();
                t.memoizedState = t.baseState = e;
                var n = {
                    pending: null,
                    lanes: 0,
                    dispatch: null,
                    lastRenderedReducer: null,
                    lastRenderedState: null
                };
                return t.queue = n, t = Ys.bind(null, ne, !0, n), n.dispatch = t, [
                    e,
                    t
                ];
            },
            useMemoCache: zs,
            useCacheRefresh: function() {
                return et().memoizedState = Im.bind(null, ne);
            },
            useEffectEvent: function(e) {
                var t = et(), n = {
                    impl: e
                };
                return t.memoizedState = n, function() {
                    if ((ye & 2) !== 0) throw Error(r(440));
                    return n.impl.apply(void 0, arguments);
                };
            }
        }, Xs = {
            readContext: Ze,
            use: Hi,
            useCallback: bf,
            useContext: Ze,
            useEffect: Vs,
            useImperativeHandle: Sf,
            useInsertionEffect: yf,
            useLayoutEffect: vf,
            useMemo: xf,
            useReducer: qi,
            useRef: hf,
            useState: function() {
                return qi(Wt);
            },
            useDebugValue: Ls,
            useDeferredValue: function(e, t) {
                var n = He();
                return _f(n, _e.memoizedState, e, t);
            },
            useTransition: function() {
                var e = qi(Wt)[0], t = He().memoizedState;
                return [
                    typeof e == "boolean" ? e : _a(e),
                    t
                ];
            },
            useSyncExternalStore: kc,
            useId: Ef,
            useHostTransitionStatus: Qs,
            useFormState: cf,
            useActionState: cf,
            useOptimistic: function(e, t) {
                var n = He();
                return nf(n, _e, e, t);
            },
            useMemoCache: zs,
            useCacheRefresh: Mf
        };
        Xs.useEffectEvent = mf;
        var Af = {
            readContext: Ze,
            use: Hi,
            useCallback: bf,
            useContext: Ze,
            useEffect: Vs,
            useImperativeHandle: Sf,
            useInsertionEffect: yf,
            useLayoutEffect: vf,
            useMemo: xf,
            useReducer: Hs,
            useRef: hf,
            useState: function() {
                return Hs(Wt);
            },
            useDebugValue: Ls,
            useDeferredValue: function(e, t) {
                var n = He();
                return _e === null ? Gs(n, e, t) : _f(n, _e.memoizedState, e, t);
            },
            useTransition: function() {
                var e = Hs(Wt)[0], t = He().memoizedState;
                return [
                    typeof e == "boolean" ? e : _a(e),
                    t
                ];
            },
            useSyncExternalStore: kc,
            useId: Ef,
            useHostTransitionStatus: Qs,
            useFormState: df,
            useActionState: df,
            useOptimistic: function(e, t) {
                var n = He();
                return _e !== null ? nf(n, _e, e, t) : (n.baseState = e, [
                    e,
                    n.queue.dispatch
                ]);
            },
            useMemoCache: zs,
            useCacheRefresh: Mf
        };
        Af.useEffectEvent = mf;
        function Fs(e, t, n, a) {
            t = e.memoizedState, n = n(a, t), n = n == null ? t : M({}, t, n), e.memoizedState = n, e.lanes === 0 && (e.updateQueue.baseState = n);
        }
        var Zs = {
            enqueueSetState: function(e, t, n) {
                e = e._reactInternals;
                var a = St(), s = vn(a);
                s.payload = t, n != null && (s.callback = n), t = pn(e, s, a), t !== null && (rt(t, e, a), pa(t, e, a));
            },
            enqueueReplaceState: function(e, t, n) {
                e = e._reactInternals;
                var a = St(), s = vn(a);
                s.tag = 1, s.payload = t, n != null && (s.callback = n), t = pn(e, s, a), t !== null && (rt(t, e, a), pa(t, e, a));
            },
            enqueueForceUpdate: function(e, t) {
                e = e._reactInternals;
                var n = St(), a = vn(n);
                a.tag = 2, t != null && (a.callback = t), t = pn(e, a, n), t !== null && (rt(t, e, n), pa(t, e, n));
            }
        };
        function zf(e, t, n, a, s, o, f) {
            return e = e.stateNode, typeof e.shouldComponentUpdate == "function" ? e.shouldComponentUpdate(a, o, f) : t.prototype && t.prototype.isPureReactComponent ? !ca(n, a) || !ca(s, o) : !0;
        }
        function Df(e, t, n, a) {
            e = t.state, typeof t.componentWillReceiveProps == "function" && t.componentWillReceiveProps(n, a), typeof t.UNSAFE_componentWillReceiveProps == "function" && t.UNSAFE_componentWillReceiveProps(n, a), t.state !== e && Zs.enqueueReplaceState(t, t.state, null);
        }
        function nl(e, t) {
            var n = t;
            if ("ref" in t) {
                n = {};
                for(var a in t)a !== "ref" && (n[a] = t[a]);
            }
            if (e = e.defaultProps) {
                n === t && (n = M({}, n));
                for(var s in e)n[s] === void 0 && (n[s] = e[s]);
            }
            return n;
        }
        function Hf(e) {
            vi(e);
        }
        function qf(e) {
            console.error(e);
        }
        function Uf(e) {
            vi(e);
        }
        function Gi(e, t) {
            try {
                var n = e.onUncaughtError;
                n(t.value, {
                    componentStack: t.stack
                });
            } catch (a) {
                setTimeout(function() {
                    throw a;
                });
            }
        }
        function Vf(e, t, n) {
            try {
                var a = e.onCaughtError;
                a(n.value, {
                    componentStack: n.stack,
                    errorBoundary: t.tag === 1 ? t.stateNode : null
                });
            } catch (s) {
                setTimeout(function() {
                    throw s;
                });
            }
        }
        function Ks(e, t, n) {
            return n = vn(n), n.tag = 3, n.payload = {
                element: null
            }, n.callback = function() {
                Gi(e, t);
            }, n;
        }
        function Lf(e) {
            return e = vn(e), e.tag = 3, e;
        }
        function Gf(e, t, n, a) {
            var s = n.type.getDerivedStateFromError;
            if (typeof s == "function") {
                var o = a.value;
                e.payload = function() {
                    return s(o);
                }, e.callback = function() {
                    Vf(t, n, a);
                };
            }
            var f = n.stateNode;
            f !== null && typeof f.componentDidCatch == "function" && (e.callback = function() {
                Vf(t, n, a), typeof s != "function" && (Rn === null ? Rn = new Set([
                    this
                ]) : Rn.add(this));
                var g = a.stack;
                this.componentDidCatch(a.value, {
                    componentStack: g !== null ? g : ""
                });
            });
        }
        function t0(e, t, n, a, s) {
            if (n.flags |= 32768, a !== null && typeof a == "object" && typeof a.then == "function") {
                if (t = n.alternate, t !== null && El(t, n, s, !0), n = mt.current, n !== null) {
                    switch(n.tag){
                        case 31:
                        case 13:
                            return jt === null ? Pi() : n.alternate === null && Ae === 0 && (Ae = 3), n.flags &= -257, n.flags |= 65536, n.lanes = s, a === Mi ? n.flags |= 16384 : (t = n.updateQueue, t === null ? n.updateQueue = new Set([
                                a
                            ]) : t.add(a), pr(e, a, s)), !1;
                        case 22:
                            return n.flags |= 65536, a === Mi ? n.flags |= 16384 : (t = n.updateQueue, t === null ? (t = {
                                transitions: null,
                                markerInstances: null,
                                retryQueue: new Set([
                                    a
                                ])
                            }, n.updateQueue = t) : (n = t.retryQueue, n === null ? t.retryQueue = new Set([
                                a
                            ]) : n.add(a)), pr(e, a, s)), !1;
                    }
                    throw Error(r(435, n.tag));
                }
                return pr(e, a, s), Pi(), !1;
            }
            if (de) return t = mt.current, t !== null ? ((t.flags & 65536) === 0 && (t.flags |= 256), t.flags |= 65536, t.lanes = s, a !== ds && (e = Error(r(422), {
                cause: a
            }), ha(Rt(e, n)))) : (a !== ds && (t = Error(r(423), {
                cause: a
            }), ha(Rt(t, n))), e = e.current.alternate, e.flags |= 65536, s &= -s, e.lanes |= s, a = Rt(a, n), s = Ks(e.stateNode, a, s), Cs(e, s), Ae !== 4 && (Ae = 2)), !1;
            var o = Error(r(520), {
                cause: a
            });
            if (o = Rt(o, n), Aa === null ? Aa = [
                o
            ] : Aa.push(o), Ae !== 4 && (Ae = 2), t === null) return !0;
            a = Rt(a, n), n = t;
            do {
                switch(n.tag){
                    case 3:
                        return n.flags |= 65536, e = s & -s, n.lanes |= e, e = Ks(n.stateNode, a, e), Cs(n, e), !1;
                    case 1:
                        if (t = n.type, o = n.stateNode, (n.flags & 128) === 0 && (typeof t.getDerivedStateFromError == "function" || o !== null && typeof o.componentDidCatch == "function" && (Rn === null || !Rn.has(o)))) return n.flags |= 65536, s &= -s, n.lanes |= s, s = Lf(s), Gf(s, e, n, a), Cs(n, s), !1;
                }
                n = n.return;
            }while (n !== null);
            return !1;
        }
        var $s = Error(r(461)), Ve = !1;
        function Ke(e, t, n, a) {
            t.child = e === null ? Yc(t, null, n, a) : el(t, e.child, n, a);
        }
        function Bf(e, t, n, a, s) {
            n = n.render;
            var o = t.ref;
            if ("ref" in a) {
                var f = {};
                for(var g in a)g !== "ref" && (f[g] = a[g]);
            } else f = a;
            return kn(t), a = Os(e, t, n, f, o, s), g = Ts(), e !== null && !Ve ? (Ns(e, t, s), Pt(e, t, s)) : (de && g && cs(t), t.flags |= 1, Ke(e, t, a, s), t.child);
        }
        function Qf(e, t, n, a, s) {
            if (e === null) {
                var o = n.type;
                return typeof o == "function" && !ss(o) && o.defaultProps === void 0 && n.compare === null ? (t.tag = 15, t.type = o, Yf(e, t, o, a, s)) : (e = xi(n.type, null, a, t, t.mode, s), e.ref = t.ref, e.return = t, t.child = e);
            }
            if (o = e.child, !nr(e, s)) {
                var f = o.memoizedProps;
                if (n = n.compare, n = n !== null ? n : ca, n(f, a) && e.ref === t.ref) return Pt(e, t, s);
            }
            return t.flags |= 1, e = Zt(o, a), e.ref = t.ref, e.return = t, t.child = e;
        }
        function Yf(e, t, n, a, s) {
            if (e !== null) {
                var o = e.memoizedProps;
                if (ca(o, a) && e.ref === t.ref) if (Ve = !1, t.pendingProps = a = o, nr(e, s)) (e.flags & 131072) !== 0 && (Ve = !0);
                else return t.lanes = e.lanes, Pt(e, t, s);
            }
            return Js(e, t, n, a, s);
        }
        function Xf(e, t, n, a) {
            var s = a.children, o = e !== null ? e.memoizedState : null;
            if (e === null && t.stateNode === null && (t.stateNode = {
                _visibility: 1,
                _pendingMarkers: null,
                _retryCache: null,
                _transitions: null
            }), a.mode === "hidden") {
                if ((t.flags & 128) !== 0) {
                    if (o = o !== null ? o.baseLanes | n : n, e !== null) {
                        for(a = t.child = e.child, s = 0; a !== null;)s = s | a.lanes | a.childLanes, a = a.sibling;
                        a = s & ~o;
                    } else a = 0, t.child = null;
                    return Ff(e, t, o, n, a);
                }
                if ((n & 536870912) !== 0) t.memoizedState = {
                    baseLanes: 0,
                    cachePool: null
                }, e !== null && wi(t, o !== null ? o.cachePool : null), o !== null ? Zc(t, o) : ws(), Kc(t);
                else return a = t.lanes = 536870912, Ff(e, t, o !== null ? o.baseLanes | n : n, n, a);
            } else o !== null ? (wi(t, o.cachePool), Zc(t, o), bn(), t.memoizedState = null) : (e !== null && wi(t, null), ws(), bn());
            return Ke(e, t, s, n), t.child;
        }
        function wa(e, t) {
            return e !== null && e.tag === 22 || t.stateNode !== null || (t.stateNode = {
                _visibility: 1,
                _pendingMarkers: null,
                _retryCache: null,
                _transitions: null
            }), t.sibling;
        }
        function Ff(e, t, n, a, s) {
            var o = Ss();
            return o = o === null ? null : {
                parent: qe._currentValue,
                pool: o
            }, t.memoizedState = {
                baseLanes: n,
                cachePool: o
            }, e !== null && wi(t, null), ws(), Kc(t), e !== null && El(e, t, a, !0), t.childLanes = s, null;
        }
        function Bi(e, t) {
            return t = Yi({
                mode: t.mode,
                children: t.children
            }, e.mode), t.ref = e.ref, e.child = t, t.return = e, t;
        }
        function Zf(e, t, n) {
            return el(t, e.child, null, n), e = Bi(t, t.pendingProps), e.flags |= 2, yt(t), t.memoizedState = null, e;
        }
        function n0(e, t, n) {
            var a = t.pendingProps, s = (t.flags & 128) !== 0;
            if (t.flags &= -129, e === null) {
                if (de) {
                    if (a.mode === "hidden") return e = Bi(t, a), t.lanes = 536870912, wa(null, e);
                    if (Ms(t), (e = je) ? (e = ah(e, Mt), e = e !== null && e.data === "&" ? e : null, e !== null && (t.memoizedState = {
                        dehydrated: e,
                        treeContext: dn !== null ? {
                            id: Vt,
                            overflow: Lt
                        } : null,
                        retryLane: 536870912,
                        hydrationErrors: null
                    }, n = jc(e), n.return = t, t.child = n, Fe = t, je = null)) : e = null, e === null) throw gn(t);
                    return t.lanes = 536870912, null;
                }
                return Bi(t, a);
            }
            var o = e.memoizedState;
            if (o !== null) {
                var f = o.dehydrated;
                if (Ms(t), s) if (t.flags & 256) t.flags &= -257, t = Zf(e, t, n);
                else if (t.memoizedState !== null) t.child = e.child, t.flags |= 128, t = null;
                else throw Error(r(558));
                else if (Ve || El(e, t, n, !1), s = (n & e.childLanes) !== 0, Ve || s) {
                    if (a = Ee, a !== null && (f = Ho(a, n), f !== 0 && f !== o.retryLane)) throw o.retryLane = f, Zn(e, f), rt(a, e, f), $s;
                    Pi(), t = Zf(e, t, n);
                } else e = o.treeContext, je = Ot(f.nextSibling), Fe = t, de = !0, hn = null, Mt = !1, e !== null && Nc(t, e), t = Bi(t, a), t.flags |= 4096;
                return t;
            }
            return e = Zt(e.child, {
                mode: a.mode,
                children: a.children
            }), e.ref = t.ref, t.child = e, e.return = t, e;
        }
        function Qi(e, t) {
            var n = t.ref;
            if (n === null) e !== null && e.ref !== null && (t.flags |= 4194816);
            else {
                if (typeof n != "function" && typeof n != "object") throw Error(r(284));
                (e === null || e.ref !== n) && (t.flags |= 4194816);
            }
        }
        function Js(e, t, n, a, s) {
            return kn(t), n = Os(e, t, n, a, void 0, s), a = Ts(), e !== null && !Ve ? (Ns(e, t, s), Pt(e, t, s)) : (de && a && cs(t), t.flags |= 1, Ke(e, t, n, s), t.child);
        }
        function Kf(e, t, n, a, s, o) {
            return kn(t), t.updateQueue = null, n = Jc(t, a, n, s), $c(e), a = Ts(), e !== null && !Ve ? (Ns(e, t, o), Pt(e, t, o)) : (de && a && cs(t), t.flags |= 1, Ke(e, t, n, o), t.child);
        }
        function $f(e, t, n, a, s) {
            if (kn(t), t.stateNode === null) {
                var o = _l, f = n.contextType;
                typeof f == "object" && f !== null && (o = Ze(f)), o = new n(a, o), t.memoizedState = o.state !== null && o.state !== void 0 ? o.state : null, o.updater = Zs, t.stateNode = o, o._reactInternals = t, o = t.stateNode, o.props = a, o.state = t.memoizedState, o.refs = {}, xs(t), f = n.contextType, o.context = typeof f == "object" && f !== null ? Ze(f) : _l, o.state = t.memoizedState, f = n.getDerivedStateFromProps, typeof f == "function" && (Fs(t, n, f, a), o.state = t.memoizedState), typeof n.getDerivedStateFromProps == "function" || typeof o.getSnapshotBeforeUpdate == "function" || typeof o.UNSAFE_componentWillMount != "function" && typeof o.componentWillMount != "function" || (f = o.state, typeof o.componentWillMount == "function" && o.componentWillMount(), typeof o.UNSAFE_componentWillMount == "function" && o.UNSAFE_componentWillMount(), f !== o.state && Zs.enqueueReplaceState(o, o.state, null), ba(t, a, o, s), Sa(), o.state = t.memoizedState), typeof o.componentDidMount == "function" && (t.flags |= 4194308), a = !0;
            } else if (e === null) {
                o = t.stateNode;
                var g = t.memoizedProps, S = nl(n, g);
                o.props = S;
                var w = o.context, N = n.contextType;
                f = _l, typeof N == "object" && N !== null && (f = Ze(N));
                var D = n.getDerivedStateFromProps;
                N = typeof D == "function" || typeof o.getSnapshotBeforeUpdate == "function", g = t.pendingProps !== g, N || typeof o.UNSAFE_componentWillReceiveProps != "function" && typeof o.componentWillReceiveProps != "function" || (g || w !== f) && Df(t, o, a, f), yn = !1;
                var E = t.memoizedState;
                o.state = E, ba(t, a, o, s), Sa(), w = t.memoizedState, g || E !== w || yn ? (typeof D == "function" && (Fs(t, n, D, a), w = t.memoizedState), (S = yn || zf(t, n, S, a, E, w, f)) ? (N || typeof o.UNSAFE_componentWillMount != "function" && typeof o.componentWillMount != "function" || (typeof o.componentWillMount == "function" && o.componentWillMount(), typeof o.UNSAFE_componentWillMount == "function" && o.UNSAFE_componentWillMount()), typeof o.componentDidMount == "function" && (t.flags |= 4194308)) : (typeof o.componentDidMount == "function" && (t.flags |= 4194308), t.memoizedProps = a, t.memoizedState = w), o.props = a, o.state = w, o.context = f, a = S) : (typeof o.componentDidMount == "function" && (t.flags |= 4194308), a = !1);
            } else {
                o = t.stateNode, _s(e, t), f = t.memoizedProps, N = nl(n, f), o.props = N, D = t.pendingProps, E = o.context, w = n.contextType, S = _l, typeof w == "object" && w !== null && (S = Ze(w)), g = n.getDerivedStateFromProps, (w = typeof g == "function" || typeof o.getSnapshotBeforeUpdate == "function") || typeof o.UNSAFE_componentWillReceiveProps != "function" && typeof o.componentWillReceiveProps != "function" || (f !== D || E !== S) && Df(t, o, a, S), yn = !1, E = t.memoizedState, o.state = E, ba(t, a, o, s), Sa();
                var O = t.memoizedState;
                f !== D || E !== O || yn || e !== null && e.dependencies !== null && Ci(e.dependencies) ? (typeof g == "function" && (Fs(t, n, g, a), O = t.memoizedState), (N = yn || zf(t, n, N, a, E, O, S) || e !== null && e.dependencies !== null && Ci(e.dependencies)) ? (w || typeof o.UNSAFE_componentWillUpdate != "function" && typeof o.componentWillUpdate != "function" || (typeof o.componentWillUpdate == "function" && o.componentWillUpdate(a, O, S), typeof o.UNSAFE_componentWillUpdate == "function" && o.UNSAFE_componentWillUpdate(a, O, S)), typeof o.componentDidUpdate == "function" && (t.flags |= 4), typeof o.getSnapshotBeforeUpdate == "function" && (t.flags |= 1024)) : (typeof o.componentDidUpdate != "function" || f === e.memoizedProps && E === e.memoizedState || (t.flags |= 4), typeof o.getSnapshotBeforeUpdate != "function" || f === e.memoizedProps && E === e.memoizedState || (t.flags |= 1024), t.memoizedProps = a, t.memoizedState = O), o.props = a, o.state = O, o.context = S, a = N) : (typeof o.componentDidUpdate != "function" || f === e.memoizedProps && E === e.memoizedState || (t.flags |= 4), typeof o.getSnapshotBeforeUpdate != "function" || f === e.memoizedProps && E === e.memoizedState || (t.flags |= 1024), a = !1);
            }
            return o = a, Qi(e, t), a = (t.flags & 128) !== 0, o || a ? (o = t.stateNode, n = a && typeof n.getDerivedStateFromError != "function" ? null : o.render(), t.flags |= 1, e !== null && a ? (t.child = el(t, e.child, null, s), t.child = el(t, null, n, s)) : Ke(e, t, n, s), t.memoizedState = o.state, e = t.child) : e = Pt(e, t, s), e;
        }
        function Jf(e, t, n, a) {
            return $n(), t.flags |= 256, Ke(e, t, n, a), t.child;
        }
        var ks = {
            dehydrated: null,
            treeContext: null,
            retryLane: 0,
            hydrationErrors: null
        };
        function Ws(e) {
            return {
                baseLanes: e,
                cachePool: Uc()
            };
        }
        function Ps(e, t, n) {
            return e = e !== null ? e.childLanes & ~n : 0, t && (e |= pt), e;
        }
        function kf(e, t, n) {
            var a = t.pendingProps, s = !1, o = (t.flags & 128) !== 0, f;
            if ((f = o) || (f = e !== null && e.memoizedState === null ? !1 : (De.current & 2) !== 0), f && (s = !0, t.flags &= -129), f = (t.flags & 32) !== 0, t.flags &= -33, e === null) {
                if (de) {
                    if (s ? Sn(t) : bn(), (e = je) ? (e = ah(e, Mt), e = e !== null && e.data !== "&" ? e : null, e !== null && (t.memoizedState = {
                        dehydrated: e,
                        treeContext: dn !== null ? {
                            id: Vt,
                            overflow: Lt
                        } : null,
                        retryLane: 536870912,
                        hydrationErrors: null
                    }, n = jc(e), n.return = t, t.child = n, Fe = t, je = null)) : e = null, e === null) throw gn(t);
                    return Dr(e) ? t.lanes = 32 : t.lanes = 536870912, null;
                }
                var g = a.children;
                return a = a.fallback, s ? (bn(), s = t.mode, g = Yi({
                    mode: "hidden",
                    children: g
                }, s), a = Kn(a, s, n, null), g.return = t, a.return = t, g.sibling = a, t.child = g, a = t.child, a.memoizedState = Ws(n), a.childLanes = Ps(e, f, n), t.memoizedState = ks, wa(null, a)) : (Sn(t), Is(t, g));
            }
            var S = e.memoizedState;
            if (S !== null && (g = S.dehydrated, g !== null)) {
                if (o) t.flags & 256 ? (Sn(t), t.flags &= -257, t = er(e, t, n)) : t.memoizedState !== null ? (bn(), t.child = e.child, t.flags |= 128, t = null) : (bn(), g = a.fallback, s = t.mode, a = Yi({
                    mode: "visible",
                    children: a.children
                }, s), g = Kn(g, s, n, null), g.flags |= 2, a.return = t, g.return = t, a.sibling = g, t.child = a, el(t, e.child, null, n), a = t.child, a.memoizedState = Ws(n), a.childLanes = Ps(e, f, n), t.memoizedState = ks, t = wa(null, a));
                else if (Sn(t), Dr(g)) {
                    if (f = g.nextSibling && g.nextSibling.dataset, f) var w = f.dgst;
                    f = w, a = Error(r(419)), a.stack = "", a.digest = f, ha({
                        value: a,
                        source: null,
                        stack: null
                    }), t = er(e, t, n);
                } else if (Ve || El(e, t, n, !1), f = (n & e.childLanes) !== 0, Ve || f) {
                    if (f = Ee, f !== null && (a = Ho(f, n), a !== 0 && a !== S.retryLane)) throw S.retryLane = a, Zn(e, a), rt(f, e, a), $s;
                    zr(g) || Pi(), t = er(e, t, n);
                } else zr(g) ? (t.flags |= 192, t.child = e.child, t = null) : (e = S.treeContext, je = Ot(g.nextSibling), Fe = t, de = !0, hn = null, Mt = !1, e !== null && Nc(t, e), t = Is(t, a.children), t.flags |= 4096);
                return t;
            }
            return s ? (bn(), g = a.fallback, s = t.mode, S = e.child, w = S.sibling, a = Zt(S, {
                mode: "hidden",
                children: a.children
            }), a.subtreeFlags = S.subtreeFlags & 65011712, w !== null ? g = Zt(w, g) : (g = Kn(g, s, n, null), g.flags |= 2), g.return = t, a.return = t, a.sibling = g, t.child = a, wa(null, a), a = t.child, g = e.child.memoizedState, g === null ? g = Ws(n) : (s = g.cachePool, s !== null ? (S = qe._currentValue, s = s.parent !== S ? {
                parent: S,
                pool: S
            } : s) : s = Uc(), g = {
                baseLanes: g.baseLanes | n,
                cachePool: s
            }), a.memoizedState = g, a.childLanes = Ps(e, f, n), t.memoizedState = ks, wa(e.child, a)) : (Sn(t), n = e.child, e = n.sibling, n = Zt(n, {
                mode: "visible",
                children: a.children
            }), n.return = t, n.sibling = null, e !== null && (f = t.deletions, f === null ? (t.deletions = [
                e
            ], t.flags |= 16) : f.push(e)), t.child = n, t.memoizedState = null, n);
        }
        function Is(e, t) {
            return t = Yi({
                mode: "visible",
                children: t
            }, e.mode), t.return = e, e.child = t;
        }
        function Yi(e, t) {
            return e = gt(22, e, null, t), e.lanes = 0, e;
        }
        function er(e, t, n) {
            return el(t, e.child, null, n), e = Is(t, t.pendingProps.children), e.flags |= 2, t.memoizedState = null, e;
        }
        function Wf(e, t, n) {
            e.lanes |= t;
            var a = e.alternate;
            a !== null && (a.lanes |= t), ms(e.return, t, n);
        }
        function tr(e, t, n, a, s, o) {
            var f = e.memoizedState;
            f === null ? e.memoizedState = {
                isBackwards: t,
                rendering: null,
                renderingStartTime: 0,
                last: a,
                tail: n,
                tailMode: s,
                treeForkCount: o
            } : (f.isBackwards = t, f.rendering = null, f.renderingStartTime = 0, f.last = a, f.tail = n, f.tailMode = s, f.treeForkCount = o);
        }
        function Pf(e, t, n) {
            var a = t.pendingProps, s = a.revealOrder, o = a.tail;
            a = a.children;
            var f = De.current, g = (f & 2) !== 0;
            if (g ? (f = f & 1 | 2, t.flags |= 128) : f &= 1, L(De, f), Ke(e, t, a, n), a = de ? da : 0, !g && e !== null && (e.flags & 128) !== 0) e: for(e = t.child; e !== null;){
                if (e.tag === 13) e.memoizedState !== null && Wf(e, n, t);
                else if (e.tag === 19) Wf(e, n, t);
                else if (e.child !== null) {
                    e.child.return = e, e = e.child;
                    continue;
                }
                if (e === t) break e;
                for(; e.sibling === null;){
                    if (e.return === null || e.return === t) break e;
                    e = e.return;
                }
                e.sibling.return = e.return, e = e.sibling;
            }
            switch(s){
                case "forwards":
                    for(n = t.child, s = null; n !== null;)e = n.alternate, e !== null && Ni(e) === null && (s = n), n = n.sibling;
                    n = s, n === null ? (s = t.child, t.child = null) : (s = n.sibling, n.sibling = null), tr(t, !1, s, n, o, a);
                    break;
                case "backwards":
                case "unstable_legacy-backwards":
                    for(n = null, s = t.child, t.child = null; s !== null;){
                        if (e = s.alternate, e !== null && Ni(e) === null) {
                            t.child = s;
                            break;
                        }
                        e = s.sibling, s.sibling = n, n = s, s = e;
                    }
                    tr(t, !0, n, null, o, a);
                    break;
                case "together":
                    tr(t, !1, null, null, void 0, a);
                    break;
                default:
                    t.memoizedState = null;
            }
            return t.child;
        }
        function Pt(e, t, n) {
            if (e !== null && (t.dependencies = e.dependencies), Cn |= t.lanes, (n & t.childLanes) === 0) if (e !== null) {
                if (El(e, t, n, !1), (n & t.childLanes) === 0) return null;
            } else return null;
            if (e !== null && t.child !== e.child) throw Error(r(153));
            if (t.child !== null) {
                for(e = t.child, n = Zt(e, e.pendingProps), t.child = n, n.return = t; e.sibling !== null;)e = e.sibling, n = n.sibling = Zt(e, e.pendingProps), n.return = t;
                n.sibling = null;
            }
            return t.child;
        }
        function nr(e, t) {
            return (e.lanes & t) !== 0 ? !0 : (e = e.dependencies, !!(e !== null && Ci(e)));
        }
        function l0(e, t, n) {
            switch(t.tag){
                case 3:
                    Ie(t, t.stateNode.containerInfo), mn(t, qe, e.memoizedState.cache), $n();
                    break;
                case 27:
                case 5:
                    Wl(t);
                    break;
                case 4:
                    Ie(t, t.stateNode.containerInfo);
                    break;
                case 10:
                    mn(t, t.type, t.memoizedProps.value);
                    break;
                case 31:
                    if (t.memoizedState !== null) return t.flags |= 128, Ms(t), null;
                    break;
                case 13:
                    var a = t.memoizedState;
                    if (a !== null) return a.dehydrated !== null ? (Sn(t), t.flags |= 128, null) : (n & t.child.childLanes) !== 0 ? kf(e, t, n) : (Sn(t), e = Pt(e, t, n), e !== null ? e.sibling : null);
                    Sn(t);
                    break;
                case 19:
                    var s = (e.flags & 128) !== 0;
                    if (a = (n & t.childLanes) !== 0, a || (El(e, t, n, !1), a = (n & t.childLanes) !== 0), s) {
                        if (a) return Pf(e, t, n);
                        t.flags |= 128;
                    }
                    if (s = t.memoizedState, s !== null && (s.rendering = null, s.tail = null, s.lastEffect = null), L(De, De.current), a) break;
                    return null;
                case 22:
                    return t.lanes = 0, Xf(e, t, n, t.pendingProps);
                case 24:
                    mn(t, qe, e.memoizedState.cache);
            }
            return Pt(e, t, n);
        }
        function If(e, t, n) {
            if (e !== null) if (e.memoizedProps !== t.pendingProps) Ve = !0;
            else {
                if (!nr(e, n) && (t.flags & 128) === 0) return Ve = !1, l0(e, t, n);
                Ve = (e.flags & 131072) !== 0;
            }
            else Ve = !1, de && (t.flags & 1048576) !== 0 && Tc(t, da, t.index);
            switch(t.lanes = 0, t.tag){
                case 16:
                    e: {
                        var a = t.pendingProps;
                        if (e = Pn(t.elementType), t.type = e, typeof e == "function") ss(e) ? (a = nl(e, a), t.tag = 1, t = $f(null, t, e, a, n)) : (t.tag = 0, t = Js(null, t, e, a, n));
                        else {
                            if (e != null) {
                                var s = e.$$typeof;
                                if (s === oe) {
                                    t.tag = 11, t = Bf(null, t, e, a, n);
                                    break e;
                                } else if (s === $) {
                                    t.tag = 14, t = Qf(null, t, e, a, n);
                                    break e;
                                }
                            }
                            throw t = Qt(e) || e, Error(r(306, t, ""));
                        }
                    }
                    return t;
                case 0:
                    return Js(e, t, t.type, t.pendingProps, n);
                case 1:
                    return a = t.type, s = nl(a, t.pendingProps), $f(e, t, a, s, n);
                case 3:
                    e: {
                        if (Ie(t, t.stateNode.containerInfo), e === null) throw Error(r(387));
                        a = t.pendingProps;
                        var o = t.memoizedState;
                        s = o.element, _s(e, t), ba(t, a, null, n);
                        var f = t.memoizedState;
                        if (a = f.cache, mn(t, qe, a), a !== o.cache && ys(t, [
                            qe
                        ], n, !0), Sa(), a = f.element, o.isDehydrated) if (o = {
                            element: a,
                            isDehydrated: !1,
                            cache: f.cache
                        }, t.updateQueue.baseState = o, t.memoizedState = o, t.flags & 256) {
                            t = Jf(e, t, a, n);
                            break e;
                        } else if (a !== s) {
                            s = Rt(Error(r(424)), t), ha(s), t = Jf(e, t, a, n);
                            break e;
                        } else for(e = t.stateNode.containerInfo, e.nodeType === 9 ? e = e.body : e = e.nodeName === "HTML" ? e.ownerDocument.body : e, je = Ot(e.firstChild), Fe = t, de = !0, hn = null, Mt = !0, n = Yc(t, null, a, n), t.child = n; n;)n.flags = n.flags & -3 | 4096, n = n.sibling;
                        else {
                            if ($n(), a === s) {
                                t = Pt(e, t, n);
                                break e;
                            }
                            Ke(e, t, a, n);
                        }
                        t = t.child;
                    }
                    return t;
                case 26:
                    return Qi(e, t), e === null ? (n = ch(t.type, null, t.pendingProps, null)) ? t.memoizedState = n : de || (n = t.type, e = t.pendingProps, a = iu(ue.current).createElement(n), a[Xe] = t, a[nt] = e, $e(a, n, e), Qe(a), t.stateNode = a) : t.memoizedState = ch(t.type, e.memoizedProps, t.pendingProps, e.memoizedState), null;
                case 27:
                    return Wl(t), e === null && de && (a = t.stateNode = sh(t.type, t.pendingProps, ue.current), Fe = t, Mt = !0, s = je, jn(t.type) ? (Hr = s, je = Ot(a.firstChild)) : je = s), Ke(e, t, t.pendingProps.children, n), Qi(e, t), e === null && (t.flags |= 4194304), t.child;
                case 5:
                    return e === null && de && ((s = a = je) && (a = z0(a, t.type, t.pendingProps, Mt), a !== null ? (t.stateNode = a, Fe = t, je = Ot(a.firstChild), Mt = !1, s = !0) : s = !1), s || gn(t)), Wl(t), s = t.type, o = t.pendingProps, f = e !== null ? e.memoizedProps : null, a = o.children, Tr(s, o) ? a = null : f !== null && Tr(s, f) && (t.flags |= 32), t.memoizedState !== null && (s = Os(e, t, $m, null, null, n), Ga._currentValue = s), Qi(e, t), Ke(e, t, a, n), t.child;
                case 6:
                    return e === null && de && ((e = n = je) && (n = D0(n, t.pendingProps, Mt), n !== null ? (t.stateNode = n, Fe = t, je = null, e = !0) : e = !1), e || gn(t)), null;
                case 13:
                    return kf(e, t, n);
                case 4:
                    return Ie(t, t.stateNode.containerInfo), a = t.pendingProps, e === null ? t.child = el(t, null, a, n) : Ke(e, t, a, n), t.child;
                case 11:
                    return Bf(e, t, t.type, t.pendingProps, n);
                case 7:
                    return Ke(e, t, t.pendingProps, n), t.child;
                case 8:
                    return Ke(e, t, t.pendingProps.children, n), t.child;
                case 12:
                    return Ke(e, t, t.pendingProps.children, n), t.child;
                case 10:
                    return a = t.pendingProps, mn(t, t.type, a.value), Ke(e, t, a.children, n), t.child;
                case 9:
                    return s = t.type._context, a = t.pendingProps.children, kn(t), s = Ze(s), a = a(s), t.flags |= 1, Ke(e, t, a, n), t.child;
                case 14:
                    return Qf(e, t, t.type, t.pendingProps, n);
                case 15:
                    return Yf(e, t, t.type, t.pendingProps, n);
                case 19:
                    return Pf(e, t, n);
                case 31:
                    return n0(e, t, n);
                case 22:
                    return Xf(e, t, n, t.pendingProps);
                case 24:
                    return kn(t), a = Ze(qe), e === null ? (s = Ss(), s === null && (s = Ee, o = vs(), s.pooledCache = o, o.refCount++, o !== null && (s.pooledCacheLanes |= n), s = o), t.memoizedState = {
                        parent: a,
                        cache: s
                    }, xs(t), mn(t, qe, s)) : ((e.lanes & n) !== 0 && (_s(e, t), ba(t, null, null, n), Sa()), s = e.memoizedState, o = t.memoizedState, s.parent !== a ? (s = {
                        parent: a,
                        cache: a
                    }, t.memoizedState = s, t.lanes === 0 && (t.memoizedState = t.updateQueue.baseState = s), mn(t, qe, a)) : (a = o.cache, mn(t, qe, a), a !== s.cache && ys(t, [
                        qe
                    ], n, !0))), Ke(e, t, t.pendingProps.children, n), t.child;
                case 29:
                    throw t.pendingProps;
            }
            throw Error(r(156, t.tag));
        }
        function It(e) {
            e.flags |= 4;
        }
        function lr(e, t, n, a, s) {
            if ((t = (e.mode & 32) !== 0) && (t = !1), t) {
                if (e.flags |= 16777216, (s & 335544128) === s) if (e.stateNode.complete) e.flags |= 8192;
                else if (Ed()) e.flags |= 8192;
                else throw In = Mi, bs;
            } else e.flags &= -16777217;
        }
        function ed(e, t) {
            if (t.type !== "stylesheet" || (t.state.loading & 4) !== 0) e.flags &= -16777217;
            else if (e.flags |= 16777216, !mh(t)) if (Ed()) e.flags |= 8192;
            else throw In = Mi, bs;
        }
        function Xi(e, t) {
            t !== null && (e.flags |= 4), e.flags & 16384 && (t = e.tag !== 22 ? Ao() : 536870912, e.lanes |= t, Vl |= t);
        }
        function Ea(e, t) {
            if (!de) switch(e.tailMode){
                case "hidden":
                    t = e.tail;
                    for(var n = null; t !== null;)t.alternate !== null && (n = t), t = t.sibling;
                    n === null ? e.tail = null : n.sibling = null;
                    break;
                case "collapsed":
                    n = e.tail;
                    for(var a = null; n !== null;)n.alternate !== null && (a = n), n = n.sibling;
                    a === null ? t || e.tail === null ? e.tail = null : e.tail.sibling = null : a.sibling = null;
            }
        }
        function Oe(e) {
            var t = e.alternate !== null && e.alternate.child === e.child, n = 0, a = 0;
            if (t) for(var s = e.child; s !== null;)n |= s.lanes | s.childLanes, a |= s.subtreeFlags & 65011712, a |= s.flags & 65011712, s.return = e, s = s.sibling;
            else for(s = e.child; s !== null;)n |= s.lanes | s.childLanes, a |= s.subtreeFlags, a |= s.flags, s.return = e, s = s.sibling;
            return e.subtreeFlags |= a, e.childLanes = n, t;
        }
        function a0(e, t, n) {
            var a = t.pendingProps;
            switch(fs(t), t.tag){
                case 16:
                case 15:
                case 0:
                case 11:
                case 7:
                case 8:
                case 12:
                case 9:
                case 14:
                    return Oe(t), null;
                case 1:
                    return Oe(t), null;
                case 3:
                    return n = t.stateNode, a = null, e !== null && (a = e.memoizedState.cache), t.memoizedState.cache !== a && (t.flags |= 2048), Jt(qe), ze(), n.pendingContext && (n.context = n.pendingContext, n.pendingContext = null), (e === null || e.child === null) && (wl(t) ? It(t) : e === null || e.memoizedState.isDehydrated && (t.flags & 256) === 0 || (t.flags |= 1024, hs())), Oe(t), null;
                case 26:
                    var s = t.type, o = t.memoizedState;
                    return e === null ? (It(t), o !== null ? (Oe(t), ed(t, o)) : (Oe(t), lr(t, s, null, a, n))) : o ? o !== e.memoizedState ? (It(t), Oe(t), ed(t, o)) : (Oe(t), t.flags &= -16777217) : (e = e.memoizedProps, e !== a && It(t), Oe(t), lr(t, s, e, a, n)), null;
                case 27:
                    if (ti(t), n = ue.current, s = t.type, e !== null && t.stateNode != null) e.memoizedProps !== a && It(t);
                    else {
                        if (!a) {
                            if (t.stateNode === null) throw Error(r(166));
                            return Oe(t), null;
                        }
                        e = Y.current, wl(t) ? Ac(t) : (e = sh(s, a, n), t.stateNode = e, It(t));
                    }
                    return Oe(t), null;
                case 5:
                    if (ti(t), s = t.type, e !== null && t.stateNode != null) e.memoizedProps !== a && It(t);
                    else {
                        if (!a) {
                            if (t.stateNode === null) throw Error(r(166));
                            return Oe(t), null;
                        }
                        if (o = Y.current, wl(t)) Ac(t);
                        else {
                            var f = iu(ue.current);
                            switch(o){
                                case 1:
                                    o = f.createElementNS("http://www.w3.org/2000/svg", s);
                                    break;
                                case 2:
                                    o = f.createElementNS("http://www.w3.org/1998/Math/MathML", s);
                                    break;
                                default:
                                    switch(s){
                                        case "svg":
                                            o = f.createElementNS("http://www.w3.org/2000/svg", s);
                                            break;
                                        case "math":
                                            o = f.createElementNS("http://www.w3.org/1998/Math/MathML", s);
                                            break;
                                        case "script":
                                            o = f.createElement("div"), o.innerHTML = "<script><\/script>", o = o.removeChild(o.firstChild);
                                            break;
                                        case "select":
                                            o = typeof a.is == "string" ? f.createElement("select", {
                                                is: a.is
                                            }) : f.createElement("select"), a.multiple ? o.multiple = !0 : a.size && (o.size = a.size);
                                            break;
                                        default:
                                            o = typeof a.is == "string" ? f.createElement(s, {
                                                is: a.is
                                            }) : f.createElement(s);
                                    }
                            }
                            o[Xe] = t, o[nt] = a;
                            e: for(f = t.child; f !== null;){
                                if (f.tag === 5 || f.tag === 6) o.appendChild(f.stateNode);
                                else if (f.tag !== 4 && f.tag !== 27 && f.child !== null) {
                                    f.child.return = f, f = f.child;
                                    continue;
                                }
                                if (f === t) break e;
                                for(; f.sibling === null;){
                                    if (f.return === null || f.return === t) break e;
                                    f = f.return;
                                }
                                f.sibling.return = f.return, f = f.sibling;
                            }
                            t.stateNode = o;
                            e: switch($e(o, s, a), s){
                                case "button":
                                case "input":
                                case "select":
                                case "textarea":
                                    a = !!a.autoFocus;
                                    break e;
                                case "img":
                                    a = !0;
                                    break e;
                                default:
                                    a = !1;
                            }
                            a && It(t);
                        }
                    }
                    return Oe(t), lr(t, t.type, e === null ? null : e.memoizedProps, t.pendingProps, n), null;
                case 6:
                    if (e && t.stateNode != null) e.memoizedProps !== a && It(t);
                    else {
                        if (typeof a != "string" && t.stateNode === null) throw Error(r(166));
                        if (e = ue.current, wl(t)) {
                            if (e = t.stateNode, n = t.memoizedProps, a = null, s = Fe, s !== null) switch(s.tag){
                                case 27:
                                case 5:
                                    a = s.memoizedProps;
                            }
                            e[Xe] = t, e = !!(e.nodeValue === n || a !== null && a.suppressHydrationWarning === !0 || kd(e.nodeValue, n)), e || gn(t, !0);
                        } else e = iu(e).createTextNode(a), e[Xe] = t, t.stateNode = e;
                    }
                    return Oe(t), null;
                case 31:
                    if (n = t.memoizedState, e === null || e.memoizedState !== null) {
                        if (a = wl(t), n !== null) {
                            if (e === null) {
                                if (!a) throw Error(r(318));
                                if (e = t.memoizedState, e = e !== null ? e.dehydrated : null, !e) throw Error(r(557));
                                e[Xe] = t;
                            } else $n(), (t.flags & 128) === 0 && (t.memoizedState = null), t.flags |= 4;
                            Oe(t), e = !1;
                        } else n = hs(), e !== null && e.memoizedState !== null && (e.memoizedState.hydrationErrors = n), e = !0;
                        if (!e) return t.flags & 256 ? (yt(t), t) : (yt(t), null);
                        if ((t.flags & 128) !== 0) throw Error(r(558));
                    }
                    return Oe(t), null;
                case 13:
                    if (a = t.memoizedState, e === null || e.memoizedState !== null && e.memoizedState.dehydrated !== null) {
                        if (s = wl(t), a !== null && a.dehydrated !== null) {
                            if (e === null) {
                                if (!s) throw Error(r(318));
                                if (s = t.memoizedState, s = s !== null ? s.dehydrated : null, !s) throw Error(r(317));
                                s[Xe] = t;
                            } else $n(), (t.flags & 128) === 0 && (t.memoizedState = null), t.flags |= 4;
                            Oe(t), s = !1;
                        } else s = hs(), e !== null && e.memoizedState !== null && (e.memoizedState.hydrationErrors = s), s = !0;
                        if (!s) return t.flags & 256 ? (yt(t), t) : (yt(t), null);
                    }
                    return yt(t), (t.flags & 128) !== 0 ? (t.lanes = n, t) : (n = a !== null, e = e !== null && e.memoizedState !== null, n && (a = t.child, s = null, a.alternate !== null && a.alternate.memoizedState !== null && a.alternate.memoizedState.cachePool !== null && (s = a.alternate.memoizedState.cachePool.pool), o = null, a.memoizedState !== null && a.memoizedState.cachePool !== null && (o = a.memoizedState.cachePool.pool), o !== s && (a.flags |= 2048)), n !== e && n && (t.child.flags |= 8192), Xi(t, t.updateQueue), Oe(t), null);
                case 4:
                    return ze(), e === null && wr(t.stateNode.containerInfo), Oe(t), null;
                case 10:
                    return Jt(t.type), Oe(t), null;
                case 19:
                    if (H(De), a = t.memoizedState, a === null) return Oe(t), null;
                    if (s = (t.flags & 128) !== 0, o = a.rendering, o === null) if (s) Ea(a, !1);
                    else {
                        if (Ae !== 0 || e !== null && (e.flags & 128) !== 0) for(e = t.child; e !== null;){
                            if (o = Ni(e), o !== null) {
                                for(t.flags |= 128, Ea(a, !1), e = o.updateQueue, t.updateQueue = e, Xi(t, e), t.subtreeFlags = 0, e = n, n = t.child; n !== null;)Mc(n, e), n = n.sibling;
                                return L(De, De.current & 1 | 2), de && Kt(t, a.treeForkCount), t.child;
                            }
                            e = e.sibling;
                        }
                        a.tail !== null && ct() > Ji && (t.flags |= 128, s = !0, Ea(a, !1), t.lanes = 4194304);
                    }
                    else {
                        if (!s) if (e = Ni(o), e !== null) {
                            if (t.flags |= 128, s = !0, e = e.updateQueue, t.updateQueue = e, Xi(t, e), Ea(a, !0), a.tail === null && a.tailMode === "hidden" && !o.alternate && !de) return Oe(t), null;
                        } else 2 * ct() - a.renderingStartTime > Ji && n !== 536870912 && (t.flags |= 128, s = !0, Ea(a, !1), t.lanes = 4194304);
                        a.isBackwards ? (o.sibling = t.child, t.child = o) : (e = a.last, e !== null ? e.sibling = o : t.child = o, a.last = o);
                    }
                    return a.tail !== null ? (e = a.tail, a.rendering = e, a.tail = e.sibling, a.renderingStartTime = ct(), e.sibling = null, n = De.current, L(De, s ? n & 1 | 2 : n & 1), de && Kt(t, a.treeForkCount), e) : (Oe(t), null);
                case 22:
                case 23:
                    return yt(t), Es(), a = t.memoizedState !== null, e !== null ? e.memoizedState !== null !== a && (t.flags |= 8192) : a && (t.flags |= 8192), a ? (n & 536870912) !== 0 && (t.flags & 128) === 0 && (Oe(t), t.subtreeFlags & 6 && (t.flags |= 8192)) : Oe(t), n = t.updateQueue, n !== null && Xi(t, n.retryQueue), n = null, e !== null && e.memoizedState !== null && e.memoizedState.cachePool !== null && (n = e.memoizedState.cachePool.pool), a = null, t.memoizedState !== null && t.memoizedState.cachePool !== null && (a = t.memoizedState.cachePool.pool), a !== n && (t.flags |= 2048), e !== null && H(Wn), null;
                case 24:
                    return n = null, e !== null && (n = e.memoizedState.cache), t.memoizedState.cache !== n && (t.flags |= 2048), Jt(qe), Oe(t), null;
                case 25:
                    return null;
                case 30:
                    return null;
            }
            throw Error(r(156, t.tag));
        }
        function i0(e, t) {
            switch(fs(t), t.tag){
                case 1:
                    return e = t.flags, e & 65536 ? (t.flags = e & -65537 | 128, t) : null;
                case 3:
                    return Jt(qe), ze(), e = t.flags, (e & 65536) !== 0 && (e & 128) === 0 ? (t.flags = e & -65537 | 128, t) : null;
                case 26:
                case 27:
                case 5:
                    return ti(t), null;
                case 31:
                    if (t.memoizedState !== null) {
                        if (yt(t), t.alternate === null) throw Error(r(340));
                        $n();
                    }
                    return e = t.flags, e & 65536 ? (t.flags = e & -65537 | 128, t) : null;
                case 13:
                    if (yt(t), e = t.memoizedState, e !== null && e.dehydrated !== null) {
                        if (t.alternate === null) throw Error(r(340));
                        $n();
                    }
                    return e = t.flags, e & 65536 ? (t.flags = e & -65537 | 128, t) : null;
                case 19:
                    return H(De), null;
                case 4:
                    return ze(), null;
                case 10:
                    return Jt(t.type), null;
                case 22:
                case 23:
                    return yt(t), Es(), e !== null && H(Wn), e = t.flags, e & 65536 ? (t.flags = e & -65537 | 128, t) : null;
                case 24:
                    return Jt(qe), null;
                case 25:
                    return null;
                default:
                    return null;
            }
        }
        function td(e, t) {
            switch(fs(t), t.tag){
                case 3:
                    Jt(qe), ze();
                    break;
                case 26:
                case 27:
                case 5:
                    ti(t);
                    break;
                case 4:
                    ze();
                    break;
                case 31:
                    t.memoizedState !== null && yt(t);
                    break;
                case 13:
                    yt(t);
                    break;
                case 19:
                    H(De);
                    break;
                case 10:
                    Jt(t.type);
                    break;
                case 22:
                case 23:
                    yt(t), Es(), e !== null && H(Wn);
                    break;
                case 24:
                    Jt(qe);
            }
        }
        function Ma(e, t) {
            try {
                var n = t.updateQueue, a = n !== null ? n.lastEffect : null;
                if (a !== null) {
                    var s = a.next;
                    n = s;
                    do {
                        if ((n.tag & e) === e) {
                            a = void 0;
                            var o = n.create, f = n.inst;
                            a = o(), f.destroy = a;
                        }
                        n = n.next;
                    }while (n !== s);
                }
            } catch (g) {
                Se(t, t.return, g);
            }
        }
        function xn(e, t, n) {
            try {
                var a = t.updateQueue, s = a !== null ? a.lastEffect : null;
                if (s !== null) {
                    var o = s.next;
                    a = o;
                    do {
                        if ((a.tag & e) === e) {
                            var f = a.inst, g = f.destroy;
                            if (g !== void 0) {
                                f.destroy = void 0, s = t;
                                var S = n, w = g;
                                try {
                                    w();
                                } catch (N) {
                                    Se(s, S, N);
                                }
                            }
                        }
                        a = a.next;
                    }while (a !== o);
                }
            } catch (N) {
                Se(t, t.return, N);
            }
        }
        function nd(e) {
            var t = e.updateQueue;
            if (t !== null) {
                var n = e.stateNode;
                try {
                    Fc(t, n);
                } catch (a) {
                    Se(e, e.return, a);
                }
            }
        }
        function ld(e, t, n) {
            n.props = nl(e.type, e.memoizedProps), n.state = e.memoizedState;
            try {
                n.componentWillUnmount();
            } catch (a) {
                Se(e, t, a);
            }
        }
        function ja(e, t) {
            try {
                var n = e.ref;
                if (n !== null) {
                    switch(e.tag){
                        case 26:
                        case 27:
                        case 5:
                            var a = e.stateNode;
                            break;
                        case 30:
                            a = e.stateNode;
                            break;
                        default:
                            a = e.stateNode;
                    }
                    typeof n == "function" ? e.refCleanup = n(a) : n.current = a;
                }
            } catch (s) {
                Se(e, t, s);
            }
        }
        function Gt(e, t) {
            var n = e.ref, a = e.refCleanup;
            if (n !== null) if (typeof a == "function") try {
                a();
            } catch (s) {
                Se(e, t, s);
            } finally{
                e.refCleanup = null, e = e.alternate, e != null && (e.refCleanup = null);
            }
            else if (typeof n == "function") try {
                n(null);
            } catch (s) {
                Se(e, t, s);
            }
            else n.current = null;
        }
        function ad(e) {
            var t = e.type, n = e.memoizedProps, a = e.stateNode;
            try {
                e: switch(t){
                    case "button":
                    case "input":
                    case "select":
                    case "textarea":
                        n.autoFocus && a.focus();
                        break e;
                    case "img":
                        n.src ? a.src = n.src : n.srcSet && (a.srcset = n.srcSet);
                }
            } catch (s) {
                Se(e, e.return, s);
            }
        }
        function ar(e, t, n) {
            try {
                var a = e.stateNode;
                M0(a, e.type, n, t), a[nt] = t;
            } catch (s) {
                Se(e, e.return, s);
            }
        }
        function id(e) {
            return e.tag === 5 || e.tag === 3 || e.tag === 26 || e.tag === 27 && jn(e.type) || e.tag === 4;
        }
        function ir(e) {
            e: for(;;){
                for(; e.sibling === null;){
                    if (e.return === null || id(e.return)) return null;
                    e = e.return;
                }
                for(e.sibling.return = e.return, e = e.sibling; e.tag !== 5 && e.tag !== 6 && e.tag !== 18;){
                    if (e.tag === 27 && jn(e.type) || e.flags & 2 || e.child === null || e.tag === 4) continue e;
                    e.child.return = e, e = e.child;
                }
                if (!(e.flags & 2)) return e.stateNode;
            }
        }
        function ur(e, t, n) {
            var a = e.tag;
            if (a === 5 || a === 6) e = e.stateNode, t ? (n.nodeType === 9 ? n.body : n.nodeName === "HTML" ? n.ownerDocument.body : n).insertBefore(e, t) : (t = n.nodeType === 9 ? n.body : n.nodeName === "HTML" ? n.ownerDocument.body : n, t.appendChild(e), n = n._reactRootContainer, n != null || t.onclick !== null || (t.onclick = Xt));
            else if (a !== 4 && (a === 27 && jn(e.type) && (n = e.stateNode, t = null), e = e.child, e !== null)) for(ur(e, t, n), e = e.sibling; e !== null;)ur(e, t, n), e = e.sibling;
        }
        function Fi(e, t, n) {
            var a = e.tag;
            if (a === 5 || a === 6) e = e.stateNode, t ? n.insertBefore(e, t) : n.appendChild(e);
            else if (a !== 4 && (a === 27 && jn(e.type) && (n = e.stateNode), e = e.child, e !== null)) for(Fi(e, t, n), e = e.sibling; e !== null;)Fi(e, t, n), e = e.sibling;
        }
        function ud(e) {
            var t = e.stateNode, n = e.memoizedProps;
            try {
                for(var a = e.type, s = t.attributes; s.length;)t.removeAttributeNode(s[0]);
                $e(t, a, n), t[Xe] = e, t[nt] = n;
            } catch (o) {
                Se(e, e.return, o);
            }
        }
        var en = !1, Le = !1, sr = !1, sd = typeof WeakSet == "function" ? WeakSet : Set, Ye = null;
        function u0(e, t) {
            if (e = e.containerInfo, jr = du, e = pc(e), es(e)) {
                if ("selectionStart" in e) var n = {
                    start: e.selectionStart,
                    end: e.selectionEnd
                };
                else e: {
                    n = (n = e.ownerDocument) && n.defaultView || window;
                    var a = n.getSelection && n.getSelection();
                    if (a && a.rangeCount !== 0) {
                        n = a.anchorNode;
                        var s = a.anchorOffset, o = a.focusNode;
                        a = a.focusOffset;
                        try {
                            n.nodeType, o.nodeType;
                        } catch  {
                            n = null;
                            break e;
                        }
                        var f = 0, g = -1, S = -1, w = 0, N = 0, D = e, E = null;
                        t: for(;;){
                            for(var O; D !== n || s !== 0 && D.nodeType !== 3 || (g = f + s), D !== o || a !== 0 && D.nodeType !== 3 || (S = f + a), D.nodeType === 3 && (f += D.nodeValue.length), (O = D.firstChild) !== null;)E = D, D = O;
                            for(;;){
                                if (D === e) break t;
                                if (E === n && ++w === s && (g = f), E === o && ++N === a && (S = f), (O = D.nextSibling) !== null) break;
                                D = E, E = D.parentNode;
                            }
                            D = O;
                        }
                        n = g === -1 || S === -1 ? null : {
                            start: g,
                            end: S
                        };
                    } else n = null;
                }
                n = n || {
                    start: 0,
                    end: 0
                };
            } else n = null;
            for(Or = {
                focusedElem: e,
                selectionRange: n
            }, du = !1, Ye = t; Ye !== null;)if (t = Ye, e = t.child, (t.subtreeFlags & 1028) !== 0 && e !== null) e.return = t, Ye = e;
            else for(; Ye !== null;){
                switch(t = Ye, o = t.alternate, e = t.flags, t.tag){
                    case 0:
                        if ((e & 4) !== 0 && (e = t.updateQueue, e = e !== null ? e.events : null, e !== null)) for(n = 0; n < e.length; n++)s = e[n], s.ref.impl = s.nextImpl;
                        break;
                    case 11:
                    case 15:
                        break;
                    case 1:
                        if ((e & 1024) !== 0 && o !== null) {
                            e = void 0, n = t, s = o.memoizedProps, o = o.memoizedState, a = n.stateNode;
                            try {
                                var Q = nl(n.type, s);
                                e = a.getSnapshotBeforeUpdate(Q, o), a.__reactInternalSnapshotBeforeUpdate = e;
                            } catch (J) {
                                Se(n, n.return, J);
                            }
                        }
                        break;
                    case 3:
                        if ((e & 1024) !== 0) {
                            if (e = t.stateNode.containerInfo, n = e.nodeType, n === 9) Ar(e);
                            else if (n === 1) switch(e.nodeName){
                                case "HEAD":
                                case "HTML":
                                case "BODY":
                                    Ar(e);
                                    break;
                                default:
                                    e.textContent = "";
                            }
                        }
                        break;
                    case 5:
                    case 26:
                    case 27:
                    case 6:
                    case 4:
                    case 17:
                        break;
                    default:
                        if ((e & 1024) !== 0) throw Error(r(163));
                }
                if (e = t.sibling, e !== null) {
                    e.return = t.return, Ye = e;
                    break;
                }
                Ye = t.return;
            }
        }
        function rd(e, t, n) {
            var a = n.flags;
            switch(n.tag){
                case 0:
                case 11:
                case 15:
                    nn(e, n), a & 4 && Ma(5, n);
                    break;
                case 1:
                    if (nn(e, n), a & 4) if (e = n.stateNode, t === null) try {
                        e.componentDidMount();
                    } catch (f) {
                        Se(n, n.return, f);
                    }
                    else {
                        var s = nl(n.type, t.memoizedProps);
                        t = t.memoizedState;
                        try {
                            e.componentDidUpdate(s, t, e.__reactInternalSnapshotBeforeUpdate);
                        } catch (f) {
                            Se(n, n.return, f);
                        }
                    }
                    a & 64 && nd(n), a & 512 && ja(n, n.return);
                    break;
                case 3:
                    if (nn(e, n), a & 64 && (e = n.updateQueue, e !== null)) {
                        if (t = null, n.child !== null) switch(n.child.tag){
                            case 27:
                            case 5:
                                t = n.child.stateNode;
                                break;
                            case 1:
                                t = n.child.stateNode;
                        }
                        try {
                            Fc(e, t);
                        } catch (f) {
                            Se(n, n.return, f);
                        }
                    }
                    break;
                case 27:
                    t === null && a & 4 && ud(n);
                case 26:
                case 5:
                    nn(e, n), t === null && a & 4 && ad(n), a & 512 && ja(n, n.return);
                    break;
                case 12:
                    nn(e, n);
                    break;
                case 31:
                    nn(e, n), a & 4 && fd(e, n);
                    break;
                case 13:
                    nn(e, n), a & 4 && dd(e, n), a & 64 && (e = n.memoizedState, e !== null && (e = e.dehydrated, e !== null && (n = m0.bind(null, n), H0(e, n))));
                    break;
                case 22:
                    if (a = n.memoizedState !== null || en, !a) {
                        t = t !== null && t.memoizedState !== null || Le, s = en;
                        var o = Le;
                        en = a, (Le = t) && !o ? ln(e, n, (n.subtreeFlags & 8772) !== 0) : nn(e, n), en = s, Le = o;
                    }
                    break;
                case 30:
                    break;
                default:
                    nn(e, n);
            }
        }
        function od(e) {
            var t = e.alternate;
            t !== null && (e.alternate = null, od(t)), e.child = null, e.deletions = null, e.sibling = null, e.tag === 5 && (t = e.stateNode, t !== null && Uu(t)), e.stateNode = null, e.return = null, e.dependencies = null, e.memoizedProps = null, e.memoizedState = null, e.pendingProps = null, e.stateNode = null, e.updateQueue = null;
        }
        var Te = null, at = !1;
        function tn(e, t, n) {
            for(n = n.child; n !== null;)cd(e, t, n), n = n.sibling;
        }
        function cd(e, t, n) {
            if (ft && typeof ft.onCommitFiberUnmount == "function") try {
                ft.onCommitFiberUnmount(Pl, n);
            } catch  {}
            switch(n.tag){
                case 26:
                    Le || Gt(n, t), tn(e, t, n), n.memoizedState ? n.memoizedState.count-- : n.stateNode && (n = n.stateNode, n.parentNode.removeChild(n));
                    break;
                case 27:
                    Le || Gt(n, t);
                    var a = Te, s = at;
                    jn(n.type) && (Te = n.stateNode, at = !1), tn(e, t, n), Ua(n.stateNode), Te = a, at = s;
                    break;
                case 5:
                    Le || Gt(n, t);
                case 6:
                    if (a = Te, s = at, Te = null, tn(e, t, n), Te = a, at = s, Te !== null) if (at) try {
                        (Te.nodeType === 9 ? Te.body : Te.nodeName === "HTML" ? Te.ownerDocument.body : Te).removeChild(n.stateNode);
                    } catch (o) {
                        Se(n, t, o);
                    }
                    else try {
                        Te.removeChild(n.stateNode);
                    } catch (o) {
                        Se(n, t, o);
                    }
                    break;
                case 18:
                    Te !== null && (at ? (e = Te, nh(e.nodeType === 9 ? e.body : e.nodeName === "HTML" ? e.ownerDocument.body : e, n.stateNode), Zl(e)) : nh(Te, n.stateNode));
                    break;
                case 4:
                    a = Te, s = at, Te = n.stateNode.containerInfo, at = !0, tn(e, t, n), Te = a, at = s;
                    break;
                case 0:
                case 11:
                case 14:
                case 15:
                    xn(2, n, t), Le || xn(4, n, t), tn(e, t, n);
                    break;
                case 1:
                    Le || (Gt(n, t), a = n.stateNode, typeof a.componentWillUnmount == "function" && ld(n, t, a)), tn(e, t, n);
                    break;
                case 21:
                    tn(e, t, n);
                    break;
                case 22:
                    Le = (a = Le) || n.memoizedState !== null, tn(e, t, n), Le = a;
                    break;
                default:
                    tn(e, t, n);
            }
        }
        function fd(e, t) {
            if (t.memoizedState === null && (e = t.alternate, e !== null && (e = e.memoizedState, e !== null))) {
                e = e.dehydrated;
                try {
                    Zl(e);
                } catch (n) {
                    Se(t, t.return, n);
                }
            }
        }
        function dd(e, t) {
            if (t.memoizedState === null && (e = t.alternate, e !== null && (e = e.memoizedState, e !== null && (e = e.dehydrated, e !== null)))) try {
                Zl(e);
            } catch (n) {
                Se(t, t.return, n);
            }
        }
        function s0(e) {
            switch(e.tag){
                case 31:
                case 13:
                case 19:
                    var t = e.stateNode;
                    return t === null && (t = e.stateNode = new sd), t;
                case 22:
                    return e = e.stateNode, t = e._retryCache, t === null && (t = e._retryCache = new sd), t;
                default:
                    throw Error(r(435, e.tag));
            }
        }
        function Zi(e, t) {
            var n = s0(e);
            t.forEach(function(a) {
                if (!n.has(a)) {
                    n.add(a);
                    var s = y0.bind(null, e, a);
                    a.then(s, s);
                }
            });
        }
        function it(e, t) {
            var n = t.deletions;
            if (n !== null) for(var a = 0; a < n.length; a++){
                var s = n[a], o = e, f = t, g = f;
                e: for(; g !== null;){
                    switch(g.tag){
                        case 27:
                            if (jn(g.type)) {
                                Te = g.stateNode, at = !1;
                                break e;
                            }
                            break;
                        case 5:
                            Te = g.stateNode, at = !1;
                            break e;
                        case 3:
                        case 4:
                            Te = g.stateNode.containerInfo, at = !0;
                            break e;
                    }
                    g = g.return;
                }
                if (Te === null) throw Error(r(160));
                cd(o, f, s), Te = null, at = !1, o = s.alternate, o !== null && (o.return = null), s.return = null;
            }
            if (t.subtreeFlags & 13886) for(t = t.child; t !== null;)hd(t, e), t = t.sibling;
        }
        var Dt = null;
        function hd(e, t) {
            var n = e.alternate, a = e.flags;
            switch(e.tag){
                case 0:
                case 11:
                case 14:
                case 15:
                    it(t, e), ut(e), a & 4 && (xn(3, e, e.return), Ma(3, e), xn(5, e, e.return));
                    break;
                case 1:
                    it(t, e), ut(e), a & 512 && (Le || n === null || Gt(n, n.return)), a & 64 && en && (e = e.updateQueue, e !== null && (a = e.callbacks, a !== null && (n = e.shared.hiddenCallbacks, e.shared.hiddenCallbacks = n === null ? a : n.concat(a))));
                    break;
                case 26:
                    var s = Dt;
                    if (it(t, e), ut(e), a & 512 && (Le || n === null || Gt(n, n.return)), a & 4) {
                        var o = n !== null ? n.memoizedState : null;
                        if (a = e.memoizedState, n === null) if (a === null) if (e.stateNode === null) {
                            e: {
                                a = e.type, n = e.memoizedProps, s = s.ownerDocument || s;
                                t: switch(a){
                                    case "title":
                                        o = s.getElementsByTagName("title")[0], (!o || o[ta] || o[Xe] || o.namespaceURI === "http://www.w3.org/2000/svg" || o.hasAttribute("itemprop")) && (o = s.createElement(a), s.head.insertBefore(o, s.querySelector("head > title"))), $e(o, a, n), o[Xe] = e, Qe(o), a = o;
                                        break e;
                                    case "link":
                                        var f = hh("link", "href", s).get(a + (n.href || ""));
                                        if (f) {
                                            for(var g = 0; g < f.length; g++)if (o = f[g], o.getAttribute("href") === (n.href == null || n.href === "" ? null : n.href) && o.getAttribute("rel") === (n.rel == null ? null : n.rel) && o.getAttribute("title") === (n.title == null ? null : n.title) && o.getAttribute("crossorigin") === (n.crossOrigin == null ? null : n.crossOrigin)) {
                                                f.splice(g, 1);
                                                break t;
                                            }
                                        }
                                        o = s.createElement(a), $e(o, a, n), s.head.appendChild(o);
                                        break;
                                    case "meta":
                                        if (f = hh("meta", "content", s).get(a + (n.content || ""))) {
                                            for(g = 0; g < f.length; g++)if (o = f[g], o.getAttribute("content") === (n.content == null ? null : "" + n.content) && o.getAttribute("name") === (n.name == null ? null : n.name) && o.getAttribute("property") === (n.property == null ? null : n.property) && o.getAttribute("http-equiv") === (n.httpEquiv == null ? null : n.httpEquiv) && o.getAttribute("charset") === (n.charSet == null ? null : n.charSet)) {
                                                f.splice(g, 1);
                                                break t;
                                            }
                                        }
                                        o = s.createElement(a), $e(o, a, n), s.head.appendChild(o);
                                        break;
                                    default:
                                        throw Error(r(468, a));
                                }
                                o[Xe] = e, Qe(o), a = o;
                            }
                            e.stateNode = a;
                        } else gh(s, e.type, e.stateNode);
                        else e.stateNode = dh(s, a, e.memoizedProps);
                        else o !== a ? (o === null ? n.stateNode !== null && (n = n.stateNode, n.parentNode.removeChild(n)) : o.count--, a === null ? gh(s, e.type, e.stateNode) : dh(s, a, e.memoizedProps)) : a === null && e.stateNode !== null && ar(e, e.memoizedProps, n.memoizedProps);
                    }
                    break;
                case 27:
                    it(t, e), ut(e), a & 512 && (Le || n === null || Gt(n, n.return)), n !== null && a & 4 && ar(e, e.memoizedProps, n.memoizedProps);
                    break;
                case 5:
                    if (it(t, e), ut(e), a & 512 && (Le || n === null || Gt(n, n.return)), e.flags & 32) {
                        s = e.stateNode;
                        try {
                            ml(s, "");
                        } catch (Q) {
                            Se(e, e.return, Q);
                        }
                    }
                    a & 4 && e.stateNode != null && (s = e.memoizedProps, ar(e, s, n !== null ? n.memoizedProps : s)), a & 1024 && (sr = !0);
                    break;
                case 6:
                    if (it(t, e), ut(e), a & 4) {
                        if (e.stateNode === null) throw Error(r(162));
                        a = e.memoizedProps, n = e.stateNode;
                        try {
                            n.nodeValue = a;
                        } catch (Q) {
                            Se(e, e.return, Q);
                        }
                    }
                    break;
                case 3:
                    if (ru = null, s = Dt, Dt = uu(t.containerInfo), it(t, e), Dt = s, ut(e), a & 4 && n !== null && n.memoizedState.isDehydrated) try {
                        Zl(t.containerInfo);
                    } catch (Q) {
                        Se(e, e.return, Q);
                    }
                    sr && (sr = !1, gd(e));
                    break;
                case 4:
                    a = Dt, Dt = uu(e.stateNode.containerInfo), it(t, e), ut(e), Dt = a;
                    break;
                case 12:
                    it(t, e), ut(e);
                    break;
                case 31:
                    it(t, e), ut(e), a & 4 && (a = e.updateQueue, a !== null && (e.updateQueue = null, Zi(e, a)));
                    break;
                case 13:
                    it(t, e), ut(e), e.child.flags & 8192 && e.memoizedState !== null != (n !== null && n.memoizedState !== null) && ($i = ct()), a & 4 && (a = e.updateQueue, a !== null && (e.updateQueue = null, Zi(e, a)));
                    break;
                case 22:
                    s = e.memoizedState !== null;
                    var S = n !== null && n.memoizedState !== null, w = en, N = Le;
                    if (en = w || s, Le = N || S, it(t, e), Le = N, en = w, ut(e), a & 8192) e: for(t = e.stateNode, t._visibility = s ? t._visibility & -2 : t._visibility | 1, s && (n === null || S || en || Le || ll(e)), n = null, t = e;;){
                        if (t.tag === 5 || t.tag === 26) {
                            if (n === null) {
                                S = n = t;
                                try {
                                    if (o = S.stateNode, s) f = o.style, typeof f.setProperty == "function" ? f.setProperty("display", "none", "important") : f.display = "none";
                                    else {
                                        g = S.stateNode;
                                        var D = S.memoizedProps.style, E = D != null && D.hasOwnProperty("display") ? D.display : null;
                                        g.style.display = E == null || typeof E == "boolean" ? "" : ("" + E).trim();
                                    }
                                } catch (Q) {
                                    Se(S, S.return, Q);
                                }
                            }
                        } else if (t.tag === 6) {
                            if (n === null) {
                                S = t;
                                try {
                                    S.stateNode.nodeValue = s ? "" : S.memoizedProps;
                                } catch (Q) {
                                    Se(S, S.return, Q);
                                }
                            }
                        } else if (t.tag === 18) {
                            if (n === null) {
                                S = t;
                                try {
                                    var O = S.stateNode;
                                    s ? lh(O, !0) : lh(S.stateNode, !1);
                                } catch (Q) {
                                    Se(S, S.return, Q);
                                }
                            }
                        } else if ((t.tag !== 22 && t.tag !== 23 || t.memoizedState === null || t === e) && t.child !== null) {
                            t.child.return = t, t = t.child;
                            continue;
                        }
                        if (t === e) break e;
                        for(; t.sibling === null;){
                            if (t.return === null || t.return === e) break e;
                            n === t && (n = null), t = t.return;
                        }
                        n === t && (n = null), t.sibling.return = t.return, t = t.sibling;
                    }
                    a & 4 && (a = e.updateQueue, a !== null && (n = a.retryQueue, n !== null && (a.retryQueue = null, Zi(e, n))));
                    break;
                case 19:
                    it(t, e), ut(e), a & 4 && (a = e.updateQueue, a !== null && (e.updateQueue = null, Zi(e, a)));
                    break;
                case 30:
                    break;
                case 21:
                    break;
                default:
                    it(t, e), ut(e);
            }
        }
        function ut(e) {
            var t = e.flags;
            if (t & 2) {
                try {
                    for(var n, a = e.return; a !== null;){
                        if (id(a)) {
                            n = a;
                            break;
                        }
                        a = a.return;
                    }
                    if (n == null) throw Error(r(160));
                    switch(n.tag){
                        case 27:
                            var s = n.stateNode, o = ir(e);
                            Fi(e, o, s);
                            break;
                        case 5:
                            var f = n.stateNode;
                            n.flags & 32 && (ml(f, ""), n.flags &= -33);
                            var g = ir(e);
                            Fi(e, g, f);
                            break;
                        case 3:
                        case 4:
                            var S = n.stateNode.containerInfo, w = ir(e);
                            ur(e, w, S);
                            break;
                        default:
                            throw Error(r(161));
                    }
                } catch (N) {
                    Se(e, e.return, N);
                }
                e.flags &= -3;
            }
            t & 4096 && (e.flags &= -4097);
        }
        function gd(e) {
            if (e.subtreeFlags & 1024) for(e = e.child; e !== null;){
                var t = e;
                gd(t), t.tag === 5 && t.flags & 1024 && t.stateNode.reset(), e = e.sibling;
            }
        }
        function nn(e, t) {
            if (t.subtreeFlags & 8772) for(t = t.child; t !== null;)rd(e, t.alternate, t), t = t.sibling;
        }
        function ll(e) {
            for(e = e.child; e !== null;){
                var t = e;
                switch(t.tag){
                    case 0:
                    case 11:
                    case 14:
                    case 15:
                        xn(4, t, t.return), ll(t);
                        break;
                    case 1:
                        Gt(t, t.return);
                        var n = t.stateNode;
                        typeof n.componentWillUnmount == "function" && ld(t, t.return, n), ll(t);
                        break;
                    case 27:
                        Ua(t.stateNode);
                    case 26:
                    case 5:
                        Gt(t, t.return), ll(t);
                        break;
                    case 22:
                        t.memoizedState === null && ll(t);
                        break;
                    case 30:
                        ll(t);
                        break;
                    default:
                        ll(t);
                }
                e = e.sibling;
            }
        }
        function ln(e, t, n) {
            for(n = n && (t.subtreeFlags & 8772) !== 0, t = t.child; t !== null;){
                var a = t.alternate, s = e, o = t, f = o.flags;
                switch(o.tag){
                    case 0:
                    case 11:
                    case 15:
                        ln(s, o, n), Ma(4, o);
                        break;
                    case 1:
                        if (ln(s, o, n), a = o, s = a.stateNode, typeof s.componentDidMount == "function") try {
                            s.componentDidMount();
                        } catch (w) {
                            Se(a, a.return, w);
                        }
                        if (a = o, s = a.updateQueue, s !== null) {
                            var g = a.stateNode;
                            try {
                                var S = s.shared.hiddenCallbacks;
                                if (S !== null) for(s.shared.hiddenCallbacks = null, s = 0; s < S.length; s++)Xc(S[s], g);
                            } catch (w) {
                                Se(a, a.return, w);
                            }
                        }
                        n && f & 64 && nd(o), ja(o, o.return);
                        break;
                    case 27:
                        ud(o);
                    case 26:
                    case 5:
                        ln(s, o, n), n && a === null && f & 4 && ad(o), ja(o, o.return);
                        break;
                    case 12:
                        ln(s, o, n);
                        break;
                    case 31:
                        ln(s, o, n), n && f & 4 && fd(s, o);
                        break;
                    case 13:
                        ln(s, o, n), n && f & 4 && dd(s, o);
                        break;
                    case 22:
                        o.memoizedState === null && ln(s, o, n), ja(o, o.return);
                        break;
                    case 30:
                        break;
                    default:
                        ln(s, o, n);
                }
                t = t.sibling;
            }
        }
        function rr(e, t) {
            var n = null;
            e !== null && e.memoizedState !== null && e.memoizedState.cachePool !== null && (n = e.memoizedState.cachePool.pool), e = null, t.memoizedState !== null && t.memoizedState.cachePool !== null && (e = t.memoizedState.cachePool.pool), e !== n && (e != null && e.refCount++, n != null && ga(n));
        }
        function or(e, t) {
            e = null, t.alternate !== null && (e = t.alternate.memoizedState.cache), t = t.memoizedState.cache, t !== e && (t.refCount++, e != null && ga(e));
        }
        function Ht(e, t, n, a) {
            if (t.subtreeFlags & 10256) for(t = t.child; t !== null;)md(e, t, n, a), t = t.sibling;
        }
        function md(e, t, n, a) {
            var s = t.flags;
            switch(t.tag){
                case 0:
                case 11:
                case 15:
                    Ht(e, t, n, a), s & 2048 && Ma(9, t);
                    break;
                case 1:
                    Ht(e, t, n, a);
                    break;
                case 3:
                    Ht(e, t, n, a), s & 2048 && (e = null, t.alternate !== null && (e = t.alternate.memoizedState.cache), t = t.memoizedState.cache, t !== e && (t.refCount++, e != null && ga(e)));
                    break;
                case 12:
                    if (s & 2048) {
                        Ht(e, t, n, a), e = t.stateNode;
                        try {
                            var o = t.memoizedProps, f = o.id, g = o.onPostCommit;
                            typeof g == "function" && g(f, t.alternate === null ? "mount" : "update", e.passiveEffectDuration, -0);
                        } catch (S) {
                            Se(t, t.return, S);
                        }
                    } else Ht(e, t, n, a);
                    break;
                case 31:
                    Ht(e, t, n, a);
                    break;
                case 13:
                    Ht(e, t, n, a);
                    break;
                case 23:
                    break;
                case 22:
                    o = t.stateNode, f = t.alternate, t.memoizedState !== null ? o._visibility & 2 ? Ht(e, t, n, a) : Oa(e, t) : o._visibility & 2 ? Ht(e, t, n, a) : (o._visibility |= 2, Hl(e, t, n, a, (t.subtreeFlags & 10256) !== 0 || !1)), s & 2048 && rr(f, t);
                    break;
                case 24:
                    Ht(e, t, n, a), s & 2048 && or(t.alternate, t);
                    break;
                default:
                    Ht(e, t, n, a);
            }
        }
        function Hl(e, t, n, a, s) {
            for(s = s && ((t.subtreeFlags & 10256) !== 0 || !1), t = t.child; t !== null;){
                var o = e, f = t, g = n, S = a, w = f.flags;
                switch(f.tag){
                    case 0:
                    case 11:
                    case 15:
                        Hl(o, f, g, S, s), Ma(8, f);
                        break;
                    case 23:
                        break;
                    case 22:
                        var N = f.stateNode;
                        f.memoizedState !== null ? N._visibility & 2 ? Hl(o, f, g, S, s) : Oa(o, f) : (N._visibility |= 2, Hl(o, f, g, S, s)), s && w & 2048 && rr(f.alternate, f);
                        break;
                    case 24:
                        Hl(o, f, g, S, s), s && w & 2048 && or(f.alternate, f);
                        break;
                    default:
                        Hl(o, f, g, S, s);
                }
                t = t.sibling;
            }
        }
        function Oa(e, t) {
            if (t.subtreeFlags & 10256) for(t = t.child; t !== null;){
                var n = e, a = t, s = a.flags;
                switch(a.tag){
                    case 22:
                        Oa(n, a), s & 2048 && rr(a.alternate, a);
                        break;
                    case 24:
                        Oa(n, a), s & 2048 && or(a.alternate, a);
                        break;
                    default:
                        Oa(n, a);
                }
                t = t.sibling;
            }
        }
        var Ta = 8192;
        function ql(e, t, n) {
            if (e.subtreeFlags & Ta) for(e = e.child; e !== null;)yd(e, t, n), e = e.sibling;
        }
        function yd(e, t, n) {
            switch(e.tag){
                case 26:
                    ql(e, t, n), e.flags & Ta && e.memoizedState !== null && K0(n, Dt, e.memoizedState, e.memoizedProps);
                    break;
                case 5:
                    ql(e, t, n);
                    break;
                case 3:
                case 4:
                    var a = Dt;
                    Dt = uu(e.stateNode.containerInfo), ql(e, t, n), Dt = a;
                    break;
                case 22:
                    e.memoizedState === null && (a = e.alternate, a !== null && a.memoizedState !== null ? (a = Ta, Ta = 16777216, ql(e, t, n), Ta = a) : ql(e, t, n));
                    break;
                default:
                    ql(e, t, n);
            }
        }
        function vd(e) {
            var t = e.alternate;
            if (t !== null && (e = t.child, e !== null)) {
                t.child = null;
                do t = e.sibling, e.sibling = null, e = t;
                while (e !== null);
            }
        }
        function Na(e) {
            var t = e.deletions;
            if ((e.flags & 16) !== 0) {
                if (t !== null) for(var n = 0; n < t.length; n++){
                    var a = t[n];
                    Ye = a, Sd(a, e);
                }
                vd(e);
            }
            if (e.subtreeFlags & 10256) for(e = e.child; e !== null;)pd(e), e = e.sibling;
        }
        function pd(e) {
            switch(e.tag){
                case 0:
                case 11:
                case 15:
                    Na(e), e.flags & 2048 && xn(9, e, e.return);
                    break;
                case 3:
                    Na(e);
                    break;
                case 12:
                    Na(e);
                    break;
                case 22:
                    var t = e.stateNode;
                    e.memoizedState !== null && t._visibility & 2 && (e.return === null || e.return.tag !== 13) ? (t._visibility &= -3, Ki(e)) : Na(e);
                    break;
                default:
                    Na(e);
            }
        }
        function Ki(e) {
            var t = e.deletions;
            if ((e.flags & 16) !== 0) {
                if (t !== null) for(var n = 0; n < t.length; n++){
                    var a = t[n];
                    Ye = a, Sd(a, e);
                }
                vd(e);
            }
            for(e = e.child; e !== null;){
                switch(t = e, t.tag){
                    case 0:
                    case 11:
                    case 15:
                        xn(8, t, t.return), Ki(t);
                        break;
                    case 22:
                        n = t.stateNode, n._visibility & 2 && (n._visibility &= -3, Ki(t));
                        break;
                    default:
                        Ki(t);
                }
                e = e.sibling;
            }
        }
        function Sd(e, t) {
            for(; Ye !== null;){
                var n = Ye;
                switch(n.tag){
                    case 0:
                    case 11:
                    case 15:
                        xn(8, n, t);
                        break;
                    case 23:
                    case 22:
                        if (n.memoizedState !== null && n.memoizedState.cachePool !== null) {
                            var a = n.memoizedState.cachePool.pool;
                            a != null && a.refCount++;
                        }
                        break;
                    case 24:
                        ga(n.memoizedState.cache);
                }
                if (a = n.child, a !== null) a.return = n, Ye = a;
                else e: for(n = e; Ye !== null;){
                    a = Ye;
                    var s = a.sibling, o = a.return;
                    if (od(a), a === n) {
                        Ye = null;
                        break e;
                    }
                    if (s !== null) {
                        s.return = o, Ye = s;
                        break e;
                    }
                    Ye = o;
                }
            }
        }
        var r0 = {
            getCacheForType: function(e) {
                var t = Ze(qe), n = t.data.get(e);
                return n === void 0 && (n = e(), t.data.set(e, n)), n;
            },
            cacheSignal: function() {
                return Ze(qe).controller.signal;
            }
        }, o0 = typeof WeakMap == "function" ? WeakMap : Map, ye = 0, Ee = null, se = null, ce = 0, pe = 0, vt = null, _n = !1, Ul = !1, cr = !1, an = 0, Ae = 0, Cn = 0, al = 0, fr = 0, pt = 0, Vl = 0, Aa = null, st = null, dr = !1, $i = 0, bd = 0, Ji = 1 / 0, ki = null, Rn = null, Ge = 0, wn = null, Ll = null, un = 0, hr = 0, gr = null, xd = null, za = 0, mr = null;
        function St() {
            return (ye & 2) !== 0 && ce !== 0 ? ce & -ce : A.T !== null ? xr() : qo();
        }
        function _d() {
            if (pt === 0) if ((ce & 536870912) === 0 || de) {
                var e = ai;
                ai <<= 1, (ai & 3932160) === 0 && (ai = 262144), pt = e;
            } else pt = 536870912;
            return e = mt.current, e !== null && (e.flags |= 32), pt;
        }
        function rt(e, t, n) {
            (e === Ee && (pe === 2 || pe === 9) || e.cancelPendingCommit !== null) && (Gl(e, 0), En(e, ce, pt, !1)), ea(e, n), ((ye & 2) === 0 || e !== Ee) && (e === Ee && ((ye & 2) === 0 && (al |= n), Ae === 4 && En(e, ce, pt, !1)), Bt(e));
        }
        function Cd(e, t, n) {
            if ((ye & 6) !== 0) throw Error(r(327));
            var a = !n && (t & 127) === 0 && (t & e.expiredLanes) === 0 || Il(e, t), s = a ? d0(e, t) : vr(e, t, !0), o = a;
            do {
                if (s === 0) {
                    Ul && !a && En(e, t, 0, !1);
                    break;
                } else {
                    if (n = e.current.alternate, o && !c0(n)) {
                        s = vr(e, t, !1), o = !1;
                        continue;
                    }
                    if (s === 2) {
                        if (o = t, e.errorRecoveryDisabledLanes & o) var f = 0;
                        else f = e.pendingLanes & -536870913, f = f !== 0 ? f : f & 536870912 ? 536870912 : 0;
                        if (f !== 0) {
                            t = f;
                            e: {
                                var g = e;
                                s = Aa;
                                var S = g.current.memoizedState.isDehydrated;
                                if (S && (Gl(g, f).flags |= 256), f = vr(g, f, !1), f !== 2) {
                                    if (cr && !S) {
                                        g.errorRecoveryDisabledLanes |= o, al |= o, s = 4;
                                        break e;
                                    }
                                    o = st, st = s, o !== null && (st === null ? st = o : st.push.apply(st, o));
                                }
                                s = f;
                            }
                            if (o = !1, s !== 2) continue;
                        }
                    }
                    if (s === 1) {
                        Gl(e, 0), En(e, t, 0, !0);
                        break;
                    }
                    e: {
                        switch(a = e, o = s, o){
                            case 0:
                            case 1:
                                throw Error(r(345));
                            case 4:
                                if ((t & 4194048) !== t) break;
                            case 6:
                                En(a, t, pt, !_n);
                                break e;
                            case 2:
                                st = null;
                                break;
                            case 3:
                            case 5:
                                break;
                            default:
                                throw Error(r(329));
                        }
                        if ((t & 62914560) === t && (s = $i + 300 - ct(), 10 < s)) {
                            if (En(a, t, pt, !_n), ui(a, 0, !0) !== 0) break e;
                            un = t, a.timeoutHandle = eh(Rd.bind(null, a, n, st, ki, dr, t, pt, al, Vl, _n, o, "Throttled", -0, 0), s);
                            break e;
                        }
                        Rd(a, n, st, ki, dr, t, pt, al, Vl, _n, o, null, -0, 0);
                    }
                }
                break;
            }while (!0);
            Bt(e);
        }
        function Rd(e, t, n, a, s, o, f, g, S, w, N, D, E, O) {
            if (e.timeoutHandle = -1, D = t.subtreeFlags, D & 8192 || (D & 16785408) === 16785408) {
                D = {
                    stylesheets: null,
                    count: 0,
                    imgCount: 0,
                    imgBytes: 0,
                    suspenseyImages: [],
                    waitingForImages: !0,
                    waitingForViewTransition: !1,
                    unsuspend: Xt
                }, yd(t, o, D);
                var Q = (o & 62914560) === o ? $i - ct() : (o & 4194048) === o ? bd - ct() : 0;
                if (Q = $0(D, Q), Q !== null) {
                    un = o, e.cancelPendingCommit = Q(Ad.bind(null, e, t, o, n, a, s, f, g, S, N, D, null, E, O)), En(e, o, f, !w);
                    return;
                }
            }
            Ad(e, t, o, n, a, s, f, g, S);
        }
        function c0(e) {
            for(var t = e;;){
                var n = t.tag;
                if ((n === 0 || n === 11 || n === 15) && t.flags & 16384 && (n = t.updateQueue, n !== null && (n = n.stores, n !== null))) for(var a = 0; a < n.length; a++){
                    var s = n[a], o = s.getSnapshot;
                    s = s.value;
                    try {
                        if (!ht(o(), s)) return !1;
                    } catch  {
                        return !1;
                    }
                }
                if (n = t.child, t.subtreeFlags & 16384 && n !== null) n.return = t, t = n;
                else {
                    if (t === e) break;
                    for(; t.sibling === null;){
                        if (t.return === null || t.return === e) return !0;
                        t = t.return;
                    }
                    t.sibling.return = t.return, t = t.sibling;
                }
            }
            return !0;
        }
        function En(e, t, n, a) {
            t &= ~fr, t &= ~al, e.suspendedLanes |= t, e.pingedLanes &= ~t, a && (e.warmLanes |= t), a = e.expirationTimes;
            for(var s = t; 0 < s;){
                var o = 31 - dt(s), f = 1 << o;
                a[o] = -1, s &= ~f;
            }
            n !== 0 && zo(e, n, t);
        }
        function Wi() {
            return (ye & 6) === 0 ? (Da(0), !1) : !0;
        }
        function yr() {
            if (se !== null) {
                if (pe === 0) var e = se.return;
                else e = se, $t = Jn = null, As(e), Tl = null, ya = 0, e = se;
                for(; e !== null;)td(e.alternate, e), e = e.return;
                se = null;
            }
        }
        function Gl(e, t) {
            var n = e.timeoutHandle;
            n !== -1 && (e.timeoutHandle = -1, T0(n)), n = e.cancelPendingCommit, n !== null && (e.cancelPendingCommit = null, n()), un = 0, yr(), Ee = e, se = n = Zt(e.current, null), ce = t, pe = 0, vt = null, _n = !1, Ul = Il(e, t), cr = !1, Vl = pt = fr = al = Cn = Ae = 0, st = Aa = null, dr = !1, (t & 8) !== 0 && (t |= t & 32);
            var a = e.entangledLanes;
            if (a !== 0) for(e = e.entanglements, a &= t; 0 < a;){
                var s = 31 - dt(a), o = 1 << s;
                t |= e[s], a &= ~o;
            }
            return an = t, pi(), n;
        }
        function wd(e, t) {
            ne = null, A.H = Ra, t === Ol || t === Ei ? (t = Gc(), pe = 3) : t === bs ? (t = Gc(), pe = 4) : pe = t === $s ? 8 : t !== null && typeof t == "object" && typeof t.then == "function" ? 6 : 1, vt = t, se === null && (Ae = 1, Gi(e, Rt(t, e.current)));
        }
        function Ed() {
            var e = mt.current;
            return e === null ? !0 : (ce & 4194048) === ce ? jt === null : (ce & 62914560) === ce || (ce & 536870912) !== 0 ? e === jt : !1;
        }
        function Md() {
            var e = A.H;
            return A.H = Ra, e === null ? Ra : e;
        }
        function jd() {
            var e = A.A;
            return A.A = r0, e;
        }
        function Pi() {
            Ae = 4, _n || (ce & 4194048) !== ce && mt.current !== null || (Ul = !0), (Cn & 134217727) === 0 && (al & 134217727) === 0 || Ee === null || En(Ee, ce, pt, !1);
        }
        function vr(e, t, n) {
            var a = ye;
            ye |= 2;
            var s = Md(), o = jd();
            (Ee !== e || ce !== t) && (ki = null, Gl(e, t)), t = !1;
            var f = Ae;
            e: do try {
                if (pe !== 0 && se !== null) {
                    var g = se, S = vt;
                    switch(pe){
                        case 8:
                            yr(), f = 6;
                            break e;
                        case 3:
                        case 2:
                        case 9:
                        case 6:
                            mt.current === null && (t = !0);
                            var w = pe;
                            if (pe = 0, vt = null, Bl(e, g, S, w), n && Ul) {
                                f = 0;
                                break e;
                            }
                            break;
                        default:
                            w = pe, pe = 0, vt = null, Bl(e, g, S, w);
                    }
                }
                f0(), f = Ae;
                break;
            } catch (N) {
                wd(e, N);
            }
            while (!0);
            return t && e.shellSuspendCounter++, $t = Jn = null, ye = a, A.H = s, A.A = o, se === null && (Ee = null, ce = 0, pi()), f;
        }
        function f0() {
            for(; se !== null;)Od(se);
        }
        function d0(e, t) {
            var n = ye;
            ye |= 2;
            var a = Md(), s = jd();
            Ee !== e || ce !== t ? (ki = null, Ji = ct() + 500, Gl(e, t)) : Ul = Il(e, t);
            e: do try {
                if (pe !== 0 && se !== null) {
                    t = se;
                    var o = vt;
                    t: switch(pe){
                        case 1:
                            pe = 0, vt = null, Bl(e, t, o, 1);
                            break;
                        case 2:
                        case 9:
                            if (Vc(o)) {
                                pe = 0, vt = null, Td(t);
                                break;
                            }
                            t = function() {
                                pe !== 2 && pe !== 9 || Ee !== e || (pe = 7), Bt(e);
                            }, o.then(t, t);
                            break e;
                        case 3:
                            pe = 7;
                            break e;
                        case 4:
                            pe = 5;
                            break e;
                        case 7:
                            Vc(o) ? (pe = 0, vt = null, Td(t)) : (pe = 0, vt = null, Bl(e, t, o, 7));
                            break;
                        case 5:
                            var f = null;
                            switch(se.tag){
                                case 26:
                                    f = se.memoizedState;
                                case 5:
                                case 27:
                                    var g = se;
                                    if (f ? mh(f) : g.stateNode.complete) {
                                        pe = 0, vt = null;
                                        var S = g.sibling;
                                        if (S !== null) se = S;
                                        else {
                                            var w = g.return;
                                            w !== null ? (se = w, Ii(w)) : se = null;
                                        }
                                        break t;
                                    }
                            }
                            pe = 0, vt = null, Bl(e, t, o, 5);
                            break;
                        case 6:
                            pe = 0, vt = null, Bl(e, t, o, 6);
                            break;
                        case 8:
                            yr(), Ae = 6;
                            break e;
                        default:
                            throw Error(r(462));
                    }
                }
                h0();
                break;
            } catch (N) {
                wd(e, N);
            }
            while (!0);
            return $t = Jn = null, A.H = a, A.A = s, ye = n, se !== null ? 0 : (Ee = null, ce = 0, pi(), Ae);
        }
        function h0() {
            for(; se !== null && !qg();)Od(se);
        }
        function Od(e) {
            var t = If(e.alternate, e, an);
            e.memoizedProps = e.pendingProps, t === null ? Ii(e) : se = t;
        }
        function Td(e) {
            var t = e, n = t.alternate;
            switch(t.tag){
                case 15:
                case 0:
                    t = Kf(n, t, t.pendingProps, t.type, void 0, ce);
                    break;
                case 11:
                    t = Kf(n, t, t.pendingProps, t.type.render, t.ref, ce);
                    break;
                case 5:
                    As(t);
                default:
                    td(n, t), t = se = Mc(t, an), t = If(n, t, an);
            }
            e.memoizedProps = e.pendingProps, t === null ? Ii(e) : se = t;
        }
        function Bl(e, t, n, a) {
            $t = Jn = null, As(t), Tl = null, ya = 0;
            var s = t.return;
            try {
                if (t0(e, s, t, n, ce)) {
                    Ae = 1, Gi(e, Rt(n, e.current)), se = null;
                    return;
                }
            } catch (o) {
                if (s !== null) throw se = s, o;
                Ae = 1, Gi(e, Rt(n, e.current)), se = null;
                return;
            }
            t.flags & 32768 ? (de || a === 1 ? e = !0 : Ul || (ce & 536870912) !== 0 ? e = !1 : (_n = e = !0, (a === 2 || a === 9 || a === 3 || a === 6) && (a = mt.current, a !== null && a.tag === 13 && (a.flags |= 16384))), Nd(t, e)) : Ii(t);
        }
        function Ii(e) {
            var t = e;
            do {
                if ((t.flags & 32768) !== 0) {
                    Nd(t, _n);
                    return;
                }
                e = t.return;
                var n = a0(t.alternate, t, an);
                if (n !== null) {
                    se = n;
                    return;
                }
                if (t = t.sibling, t !== null) {
                    se = t;
                    return;
                }
                se = t = e;
            }while (t !== null);
            Ae === 0 && (Ae = 5);
        }
        function Nd(e, t) {
            do {
                var n = i0(e.alternate, e);
                if (n !== null) {
                    n.flags &= 32767, se = n;
                    return;
                }
                if (n = e.return, n !== null && (n.flags |= 32768, n.subtreeFlags = 0, n.deletions = null), !t && (e = e.sibling, e !== null)) {
                    se = e;
                    return;
                }
                se = e = n;
            }while (e !== null);
            Ae = 6, se = null;
        }
        function Ad(e, t, n, a, s, o, f, g, S) {
            e.cancelPendingCommit = null;
            do eu();
            while (Ge !== 0);
            if ((ye & 6) !== 0) throw Error(r(327));
            if (t !== null) {
                if (t === e.current) throw Error(r(177));
                if (o = t.lanes | t.childLanes, o |= is, Zg(e, n, o, f, g, S), e === Ee && (se = Ee = null, ce = 0), Ll = t, wn = e, un = n, hr = o, gr = s, xd = a, (t.subtreeFlags & 10256) !== 0 || (t.flags & 10256) !== 0 ? (e.callbackNode = null, e.callbackPriority = 0, v0(ni, function() {
                    return Ud(), null;
                })) : (e.callbackNode = null, e.callbackPriority = 0), a = (t.flags & 13878) !== 0, (t.subtreeFlags & 13878) !== 0 || a) {
                    a = A.T, A.T = null, s = V.p, V.p = 2, f = ye, ye |= 4;
                    try {
                        u0(e, t, n);
                    } finally{
                        ye = f, V.p = s, A.T = a;
                    }
                }
                Ge = 1, zd(), Dd(), Hd();
            }
        }
        function zd() {
            if (Ge === 1) {
                Ge = 0;
                var e = wn, t = Ll, n = (t.flags & 13878) !== 0;
                if ((t.subtreeFlags & 13878) !== 0 || n) {
                    n = A.T, A.T = null;
                    var a = V.p;
                    V.p = 2;
                    var s = ye;
                    ye |= 4;
                    try {
                        hd(t, e);
                        var o = Or, f = pc(e.containerInfo), g = o.focusedElem, S = o.selectionRange;
                        if (f !== g && g && g.ownerDocument && vc(g.ownerDocument.documentElement, g)) {
                            if (S !== null && es(g)) {
                                var w = S.start, N = S.end;
                                if (N === void 0 && (N = w), "selectionStart" in g) g.selectionStart = w, g.selectionEnd = Math.min(N, g.value.length);
                                else {
                                    var D = g.ownerDocument || document, E = D && D.defaultView || window;
                                    if (E.getSelection) {
                                        var O = E.getSelection(), Q = g.textContent.length, J = Math.min(S.start, Q), Re = S.end === void 0 ? J : Math.min(S.end, Q);
                                        !O.extend && J > Re && (f = Re, Re = J, J = f);
                                        var C = yc(g, J), b = yc(g, Re);
                                        if (C && b && (O.rangeCount !== 1 || O.anchorNode !== C.node || O.anchorOffset !== C.offset || O.focusNode !== b.node || O.focusOffset !== b.offset)) {
                                            var R = D.createRange();
                                            R.setStart(C.node, C.offset), O.removeAllRanges(), J > Re ? (O.addRange(R), O.extend(b.node, b.offset)) : (R.setEnd(b.node, b.offset), O.addRange(R));
                                        }
                                    }
                                }
                            }
                            for(D = [], O = g; O = O.parentNode;)O.nodeType === 1 && D.push({
                                element: O,
                                left: O.scrollLeft,
                                top: O.scrollTop
                            });
                            for(typeof g.focus == "function" && g.focus(), g = 0; g < D.length; g++){
                                var z = D[g];
                                z.element.scrollLeft = z.left, z.element.scrollTop = z.top;
                            }
                        }
                        du = !!jr, Or = jr = null;
                    } finally{
                        ye = s, V.p = a, A.T = n;
                    }
                }
                e.current = t, Ge = 2;
            }
        }
        function Dd() {
            if (Ge === 2) {
                Ge = 0;
                var e = wn, t = Ll, n = (t.flags & 8772) !== 0;
                if ((t.subtreeFlags & 8772) !== 0 || n) {
                    n = A.T, A.T = null;
                    var a = V.p;
                    V.p = 2;
                    var s = ye;
                    ye |= 4;
                    try {
                        rd(e, t.alternate, t);
                    } finally{
                        ye = s, V.p = a, A.T = n;
                    }
                }
                Ge = 3;
            }
        }
        function Hd() {
            if (Ge === 4 || Ge === 3) {
                Ge = 0, Ug();
                var e = wn, t = Ll, n = un, a = xd;
                (t.subtreeFlags & 10256) !== 0 || (t.flags & 10256) !== 0 ? Ge = 5 : (Ge = 0, Ll = wn = null, qd(e, e.pendingLanes));
                var s = e.pendingLanes;
                if (s === 0 && (Rn = null), Hu(n), t = t.stateNode, ft && typeof ft.onCommitFiberRoot == "function") try {
                    ft.onCommitFiberRoot(Pl, t, void 0, (t.current.flags & 128) === 128);
                } catch  {}
                if (a !== null) {
                    t = A.T, s = V.p, V.p = 2, A.T = null;
                    try {
                        for(var o = e.onRecoverableError, f = 0; f < a.length; f++){
                            var g = a[f];
                            o(g.value, {
                                componentStack: g.stack
                            });
                        }
                    } finally{
                        A.T = t, V.p = s;
                    }
                }
                (un & 3) !== 0 && eu(), Bt(e), s = e.pendingLanes, (n & 261930) !== 0 && (s & 42) !== 0 ? e === mr ? za++ : (za = 0, mr = e) : za = 0, Da(0);
            }
        }
        function qd(e, t) {
            (e.pooledCacheLanes &= t) === 0 && (t = e.pooledCache, t != null && (e.pooledCache = null, ga(t)));
        }
        function eu() {
            return zd(), Dd(), Hd(), Ud();
        }
        function Ud() {
            if (Ge !== 5) return !1;
            var e = wn, t = hr;
            hr = 0;
            var n = Hu(un), a = A.T, s = V.p;
            try {
                V.p = 32 > n ? 32 : n, A.T = null, n = gr, gr = null;
                var o = wn, f = un;
                if (Ge = 0, Ll = wn = null, un = 0, (ye & 6) !== 0) throw Error(r(331));
                var g = ye;
                if (ye |= 4, pd(o.current), md(o, o.current, f, n), ye = g, Da(0, !1), ft && typeof ft.onPostCommitFiberRoot == "function") try {
                    ft.onPostCommitFiberRoot(Pl, o);
                } catch  {}
                return !0;
            } finally{
                V.p = s, A.T = a, qd(e, t);
            }
        }
        function Vd(e, t, n) {
            t = Rt(n, t), t = Ks(e.stateNode, t, 2), e = pn(e, t, 2), e !== null && (ea(e, 2), Bt(e));
        }
        function Se(e, t, n) {
            if (e.tag === 3) Vd(e, e, n);
            else for(; t !== null;){
                if (t.tag === 3) {
                    Vd(t, e, n);
                    break;
                } else if (t.tag === 1) {
                    var a = t.stateNode;
                    if (typeof t.type.getDerivedStateFromError == "function" || typeof a.componentDidCatch == "function" && (Rn === null || !Rn.has(a))) {
                        e = Rt(n, e), n = Lf(2), a = pn(t, n, 2), a !== null && (Gf(n, a, t, e), ea(a, 2), Bt(a));
                        break;
                    }
                }
                t = t.return;
            }
        }
        function pr(e, t, n) {
            var a = e.pingCache;
            if (a === null) {
                a = e.pingCache = new o0;
                var s = new Set;
                a.set(t, s);
            } else s = a.get(t), s === void 0 && (s = new Set, a.set(t, s));
            s.has(n) || (cr = !0, s.add(n), e = g0.bind(null, e, t, n), t.then(e, e));
        }
        function g0(e, t, n) {
            var a = e.pingCache;
            a !== null && a.delete(t), e.pingedLanes |= e.suspendedLanes & n, e.warmLanes &= ~n, Ee === e && (ce & n) === n && (Ae === 4 || Ae === 3 && (ce & 62914560) === ce && 300 > ct() - $i ? (ye & 2) === 0 && Gl(e, 0) : fr |= n, Vl === ce && (Vl = 0)), Bt(e);
        }
        function Ld(e, t) {
            t === 0 && (t = Ao()), e = Zn(e, t), e !== null && (ea(e, t), Bt(e));
        }
        function m0(e) {
            var t = e.memoizedState, n = 0;
            t !== null && (n = t.retryLane), Ld(e, n);
        }
        function y0(e, t) {
            var n = 0;
            switch(e.tag){
                case 31:
                case 13:
                    var a = e.stateNode, s = e.memoizedState;
                    s !== null && (n = s.retryLane);
                    break;
                case 19:
                    a = e.stateNode;
                    break;
                case 22:
                    a = e.stateNode._retryCache;
                    break;
                default:
                    throw Error(r(314));
            }
            a !== null && a.delete(t), Ld(e, n);
        }
        function v0(e, t) {
            return Nu(e, t);
        }
        var tu = null, Ql = null, Sr = !1, nu = !1, br = !1, Mn = 0;
        function Bt(e) {
            e !== Ql && e.next === null && (Ql === null ? tu = Ql = e : Ql = Ql.next = e), nu = !0, Sr || (Sr = !0, S0());
        }
        function Da(e, t) {
            if (!br && nu) {
                br = !0;
                do for(var n = !1, a = tu; a !== null;){
                    if (e !== 0) {
                        var s = a.pendingLanes;
                        if (s === 0) var o = 0;
                        else {
                            var f = a.suspendedLanes, g = a.pingedLanes;
                            o = (1 << 31 - dt(42 | e) + 1) - 1, o &= s & ~(f & ~g), o = o & 201326741 ? o & 201326741 | 1 : o ? o | 2 : 0;
                        }
                        o !== 0 && (n = !0, Yd(a, o));
                    } else o = ce, o = ui(a, a === Ee ? o : 0, a.cancelPendingCommit !== null || a.timeoutHandle !== -1), (o & 3) === 0 || Il(a, o) || (n = !0, Yd(a, o));
                    a = a.next;
                }
                while (n);
                br = !1;
            }
        }
        function p0() {
            Gd();
        }
        function Gd() {
            nu = Sr = !1;
            var e = 0;
            Mn !== 0 && O0() && (e = Mn);
            for(var t = ct(), n = null, a = tu; a !== null;){
                var s = a.next, o = Bd(a, t);
                o === 0 ? (a.next = null, n === null ? tu = s : n.next = s, s === null && (Ql = n)) : (n = a, (e !== 0 || (o & 3) !== 0) && (nu = !0)), a = s;
            }
            Ge !== 0 && Ge !== 5 || Da(e), Mn !== 0 && (Mn = 0);
        }
        function Bd(e, t) {
            for(var n = e.suspendedLanes, a = e.pingedLanes, s = e.expirationTimes, o = e.pendingLanes & -62914561; 0 < o;){
                var f = 31 - dt(o), g = 1 << f, S = s[f];
                S === -1 ? ((g & n) === 0 || (g & a) !== 0) && (s[f] = Fg(g, t)) : S <= t && (e.expiredLanes |= g), o &= ~g;
            }
            if (t = Ee, n = ce, n = ui(e, e === t ? n : 0, e.cancelPendingCommit !== null || e.timeoutHandle !== -1), a = e.callbackNode, n === 0 || e === t && (pe === 2 || pe === 9) || e.cancelPendingCommit !== null) return a !== null && a !== null && Au(a), e.callbackNode = null, e.callbackPriority = 0;
            if ((n & 3) === 0 || Il(e, n)) {
                if (t = n & -n, t === e.callbackPriority) return t;
                switch(a !== null && Au(a), Hu(n)){
                    case 2:
                    case 8:
                        n = To;
                        break;
                    case 32:
                        n = ni;
                        break;
                    case 268435456:
                        n = No;
                        break;
                    default:
                        n = ni;
                }
                return a = Qd.bind(null, e), n = Nu(n, a), e.callbackPriority = t, e.callbackNode = n, t;
            }
            return a !== null && a !== null && Au(a), e.callbackPriority = 2, e.callbackNode = null, 2;
        }
        function Qd(e, t) {
            if (Ge !== 0 && Ge !== 5) return e.callbackNode = null, e.callbackPriority = 0, null;
            var n = e.callbackNode;
            if (eu() && e.callbackNode !== n) return null;
            var a = ce;
            return a = ui(e, e === Ee ? a : 0, e.cancelPendingCommit !== null || e.timeoutHandle !== -1), a === 0 ? null : (Cd(e, a, t), Bd(e, ct()), e.callbackNode != null && e.callbackNode === n ? Qd.bind(null, e) : null);
        }
        function Yd(e, t) {
            if (eu()) return null;
            Cd(e, t, !0);
        }
        function S0() {
            N0(function() {
                (ye & 6) !== 0 ? Nu(Oo, p0) : Gd();
            });
        }
        function xr() {
            if (Mn === 0) {
                var e = Ml;
                e === 0 && (e = li, li <<= 1, (li & 261888) === 0 && (li = 256)), Mn = e;
            }
            return Mn;
        }
        function Xd(e) {
            return e == null || typeof e == "symbol" || typeof e == "boolean" ? null : typeof e == "function" ? e : ci("" + e);
        }
        function Fd(e, t) {
            var n = t.ownerDocument.createElement("input");
            return n.name = t.name, n.value = t.value, e.id && n.setAttribute("form", e.id), t.parentNode.insertBefore(n, t), e = new FormData(e), n.parentNode.removeChild(n), e;
        }
        function b0(e, t, n, a, s) {
            if (t === "submit" && n && n.stateNode === s) {
                var o = Xd((s[nt] || null).action), f = a.submitter;
                f && (t = (t = f[nt] || null) ? Xd(t.formAction) : f.getAttribute("formAction"), t !== null && (o = t, f = null));
                var g = new gi("action", "action", null, a, s);
                e.push({
                    event: g,
                    listeners: [
                        {
                            instance: null,
                            listener: function() {
                                if (a.defaultPrevented) {
                                    if (Mn !== 0) {
                                        var S = f ? Fd(s, f) : new FormData(s);
                                        Bs(n, {
                                            pending: !0,
                                            data: S,
                                            method: s.method,
                                            action: o
                                        }, null, S);
                                    }
                                } else typeof o == "function" && (g.preventDefault(), S = f ? Fd(s, f) : new FormData(s), Bs(n, {
                                    pending: !0,
                                    data: S,
                                    method: s.method,
                                    action: o
                                }, o, S));
                            },
                            currentTarget: s
                        }
                    ]
                });
            }
        }
        for(var _r = 0; _r < as.length; _r++){
            var Cr = as[_r], x0 = Cr.toLowerCase(), _0 = Cr[0].toUpperCase() + Cr.slice(1);
            zt(x0, "on" + _0);
        }
        zt(xc, "onAnimationEnd"), zt(_c, "onAnimationIteration"), zt(Cc, "onAnimationStart"), zt("dblclick", "onDoubleClick"), zt("focusin", "onFocus"), zt("focusout", "onBlur"), zt(Vm, "onTransitionRun"), zt(Lm, "onTransitionStart"), zt(Gm, "onTransitionCancel"), zt(Rc, "onTransitionEnd"), hl("onMouseEnter", [
            "mouseout",
            "mouseover"
        ]), hl("onMouseLeave", [
            "mouseout",
            "mouseover"
        ]), hl("onPointerEnter", [
            "pointerout",
            "pointerover"
        ]), hl("onPointerLeave", [
            "pointerout",
            "pointerover"
        ]), Qn("onChange", "change click focusin focusout input keydown keyup selectionchange".split(" ")), Qn("onSelect", "focusout contextmenu dragend focusin keydown keyup mousedown mouseup selectionchange".split(" ")), Qn("onBeforeInput", [
            "compositionend",
            "keypress",
            "textInput",
            "paste"
        ]), Qn("onCompositionEnd", "compositionend focusout keydown keypress keyup mousedown".split(" ")), Qn("onCompositionStart", "compositionstart focusout keydown keypress keyup mousedown".split(" ")), Qn("onCompositionUpdate", "compositionupdate focusout keydown keypress keyup mousedown".split(" "));
        var Ha = "abort canplay canplaythrough durationchange emptied encrypted ended error loadeddata loadedmetadata loadstart pause play playing progress ratechange resize seeked seeking stalled suspend timeupdate volumechange waiting".split(" "), C0 = new Set("beforetoggle cancel close invalid load scroll scrollend toggle".split(" ").concat(Ha));
        function Zd(e, t) {
            t = (t & 4) !== 0;
            for(var n = 0; n < e.length; n++){
                var a = e[n], s = a.event;
                a = a.listeners;
                e: {
                    var o = void 0;
                    if (t) for(var f = a.length - 1; 0 <= f; f--){
                        var g = a[f], S = g.instance, w = g.currentTarget;
                        if (g = g.listener, S !== o && s.isPropagationStopped()) break e;
                        o = g, s.currentTarget = w;
                        try {
                            o(s);
                        } catch (N) {
                            vi(N);
                        }
                        s.currentTarget = null, o = S;
                    }
                    else for(f = 0; f < a.length; f++){
                        if (g = a[f], S = g.instance, w = g.currentTarget, g = g.listener, S !== o && s.isPropagationStopped()) break e;
                        o = g, s.currentTarget = w;
                        try {
                            o(s);
                        } catch (N) {
                            vi(N);
                        }
                        s.currentTarget = null, o = S;
                    }
                }
            }
        }
        function re(e, t) {
            var n = t[qu];
            n === void 0 && (n = t[qu] = new Set);
            var a = e + "__bubble";
            n.has(a) || (Kd(t, e, 2, !1), n.add(a));
        }
        function Rr(e, t, n) {
            var a = 0;
            t && (a |= 4), Kd(n, e, a, t);
        }
        var lu = "_reactListening" + Math.random().toString(36).slice(2);
        function wr(e) {
            if (!e[lu]) {
                e[lu] = !0, Lo.forEach(function(n) {
                    n !== "selectionchange" && (C0.has(n) || Rr(n, !1, e), Rr(n, !0, e));
                });
                var t = e.nodeType === 9 ? e : e.ownerDocument;
                t === null || t[lu] || (t[lu] = !0, Rr("selectionchange", !1, t));
            }
        }
        function Kd(e, t, n, a) {
            switch(_h(t)){
                case 2:
                    var s = W0;
                    break;
                case 8:
                    s = P0;
                    break;
                default:
                    s = Gr;
            }
            n = s.bind(null, t, n, e), s = void 0, !Fu || t !== "touchstart" && t !== "touchmove" && t !== "wheel" || (s = !0), a ? s !== void 0 ? e.addEventListener(t, n, {
                capture: !0,
                passive: s
            }) : e.addEventListener(t, n, !0) : s !== void 0 ? e.addEventListener(t, n, {
                passive: s
            }) : e.addEventListener(t, n, !1);
        }
        function Er(e, t, n, a, s) {
            var o = a;
            if ((t & 1) === 0 && (t & 2) === 0 && a !== null) e: for(;;){
                if (a === null) return;
                var f = a.tag;
                if (f === 3 || f === 4) {
                    var g = a.stateNode.containerInfo;
                    if (g === s) break;
                    if (f === 4) for(f = a.return; f !== null;){
                        var S = f.tag;
                        if ((S === 3 || S === 4) && f.stateNode.containerInfo === s) return;
                        f = f.return;
                    }
                    for(; g !== null;){
                        if (f = cl(g), f === null) return;
                        if (S = f.tag, S === 5 || S === 6 || S === 26 || S === 27) {
                            a = o = f;
                            continue e;
                        }
                        g = g.parentNode;
                    }
                }
                a = a.return;
            }
            Wo(function() {
                var w = o, N = Yu(n), D = [];
                e: {
                    var E = wc.get(e);
                    if (E !== void 0) {
                        var O = gi, Q = e;
                        switch(e){
                            case "keypress":
                                if (di(n) === 0) break e;
                            case "keydown":
                            case "keyup":
                                O = ym;
                                break;
                            case "focusin":
                                Q = "focus", O = Ju;
                                break;
                            case "focusout":
                                Q = "blur", O = Ju;
                                break;
                            case "beforeblur":
                            case "afterblur":
                                O = Ju;
                                break;
                            case "click":
                                if (n.button === 2) break e;
                            case "auxclick":
                            case "dblclick":
                            case "mousedown":
                            case "mousemove":
                            case "mouseup":
                            case "mouseout":
                            case "mouseover":
                            case "contextmenu":
                                O = ec;
                                break;
                            case "drag":
                            case "dragend":
                            case "dragenter":
                            case "dragexit":
                            case "dragleave":
                            case "dragover":
                            case "dragstart":
                            case "drop":
                                O = am;
                                break;
                            case "touchcancel":
                            case "touchend":
                            case "touchmove":
                            case "touchstart":
                                O = Sm;
                                break;
                            case xc:
                            case _c:
                            case Cc:
                                O = sm;
                                break;
                            case Rc:
                                O = xm;
                                break;
                            case "scroll":
                            case "scrollend":
                                O = nm;
                                break;
                            case "wheel":
                                O = Cm;
                                break;
                            case "copy":
                            case "cut":
                            case "paste":
                                O = om;
                                break;
                            case "gotpointercapture":
                            case "lostpointercapture":
                            case "pointercancel":
                            case "pointerdown":
                            case "pointermove":
                            case "pointerout":
                            case "pointerover":
                            case "pointerup":
                                O = nc;
                                break;
                            case "toggle":
                            case "beforetoggle":
                                O = wm;
                        }
                        var J = (t & 4) !== 0, Re = !J && (e === "scroll" || e === "scrollend"), C = J ? E !== null ? E + "Capture" : null : E;
                        J = [];
                        for(var b = w, R; b !== null;){
                            var z = b;
                            if (R = z.stateNode, z = z.tag, z !== 5 && z !== 26 && z !== 27 || R === null || C === null || (z = la(b, C), z != null && J.push(qa(b, z, R))), Re) break;
                            b = b.return;
                        }
                        0 < J.length && (E = new O(E, Q, null, n, N), D.push({
                            event: E,
                            listeners: J
                        }));
                    }
                }
                if ((t & 7) === 0) {
                    e: {
                        if (E = e === "mouseover" || e === "pointerover", O = e === "mouseout" || e === "pointerout", E && n !== Qu && (Q = n.relatedTarget || n.fromElement) && (cl(Q) || Q[ol])) break e;
                        if ((O || E) && (E = N.window === N ? N : (E = N.ownerDocument) ? E.defaultView || E.parentWindow : window, O ? (Q = n.relatedTarget || n.toElement, O = w, Q = Q ? cl(Q) : null, Q !== null && (Re = d(Q), J = Q.tag, Q !== Re || J !== 5 && J !== 27 && J !== 6) && (Q = null)) : (O = null, Q = w), O !== Q)) {
                            if (J = ec, z = "onMouseLeave", C = "onMouseEnter", b = "mouse", (e === "pointerout" || e === "pointerover") && (J = nc, z = "onPointerLeave", C = "onPointerEnter", b = "pointer"), Re = O == null ? E : na(O), R = Q == null ? E : na(Q), E = new J(z, b + "leave", O, n, N), E.target = Re, E.relatedTarget = R, z = null, cl(N) === w && (J = new J(C, b + "enter", Q, n, N), J.target = R, J.relatedTarget = Re, z = J), Re = z, O && Q) t: {
                                for(J = R0, C = O, b = Q, R = 0, z = C; z; z = J(z))R++;
                                z = 0;
                                for(var K = b; K; K = J(K))z++;
                                for(; 0 < R - z;)C = J(C), R--;
                                for(; 0 < z - R;)b = J(b), z--;
                                for(; R--;){
                                    if (C === b || b !== null && C === b.alternate) {
                                        J = C;
                                        break t;
                                    }
                                    C = J(C), b = J(b);
                                }
                                J = null;
                            }
                            else J = null;
                            O !== null && $d(D, E, O, J, !1), Q !== null && Re !== null && $d(D, Re, Q, J, !0);
                        }
                    }
                    e: {
                        if (E = w ? na(w) : window, O = E.nodeName && E.nodeName.toLowerCase(), O === "select" || O === "input" && E.type === "file") var ge = cc;
                        else if (rc(E)) if (fc) ge = Hm;
                        else {
                            ge = zm;
                            var X = Am;
                        }
                        else O = E.nodeName, !O || O.toLowerCase() !== "input" || E.type !== "checkbox" && E.type !== "radio" ? w && Bu(w.elementType) && (ge = cc) : ge = Dm;
                        if (ge && (ge = ge(e, w))) {
                            oc(D, ge, n, N);
                            break e;
                        }
                        X && X(e, E, w), e === "focusout" && w && E.type === "number" && w.memoizedProps.value != null && Gu(E, "number", E.value);
                    }
                    switch(X = w ? na(w) : window, e){
                        case "focusin":
                            (rc(X) || X.contentEditable === "true") && (Sl = X, ts = w, fa = null);
                            break;
                        case "focusout":
                            fa = ts = Sl = null;
                            break;
                        case "mousedown":
                            ns = !0;
                            break;
                        case "contextmenu":
                        case "mouseup":
                        case "dragend":
                            ns = !1, Sc(D, n, N);
                            break;
                        case "selectionchange":
                            if (Um) break;
                        case "keydown":
                        case "keyup":
                            Sc(D, n, N);
                    }
                    var le;
                    if (Wu) e: {
                        switch(e){
                            case "compositionstart":
                                var fe = "onCompositionStart";
                                break e;
                            case "compositionend":
                                fe = "onCompositionEnd";
                                break e;
                            case "compositionupdate":
                                fe = "onCompositionUpdate";
                                break e;
                        }
                        fe = void 0;
                    }
                    else pl ? uc(e, n) && (fe = "onCompositionEnd") : e === "keydown" && n.keyCode === 229 && (fe = "onCompositionStart");
                    fe && (lc && n.locale !== "ko" && (pl || fe !== "onCompositionStart" ? fe === "onCompositionEnd" && pl && (le = Po()) : (fn = N, Zu = "value" in fn ? fn.value : fn.textContent, pl = !0)), X = au(w, fe), 0 < X.length && (fe = new tc(fe, e, null, n, N), D.push({
                        event: fe,
                        listeners: X
                    }), le ? fe.data = le : (le = sc(n), le !== null && (fe.data = le)))), (le = Mm ? jm(e, n) : Om(e, n)) && (fe = au(w, "onBeforeInput"), 0 < fe.length && (X = new tc("onBeforeInput", "beforeinput", null, n, N), D.push({
                        event: X,
                        listeners: fe
                    }), X.data = le)), b0(D, e, w, n, N);
                }
                Zd(D, t);
            });
        }
        function qa(e, t, n) {
            return {
                instance: e,
                listener: t,
                currentTarget: n
            };
        }
        function au(e, t) {
            for(var n = t + "Capture", a = []; e !== null;){
                var s = e, o = s.stateNode;
                if (s = s.tag, s !== 5 && s !== 26 && s !== 27 || o === null || (s = la(e, n), s != null && a.unshift(qa(e, s, o)), s = la(e, t), s != null && a.push(qa(e, s, o))), e.tag === 3) return a;
                e = e.return;
            }
            return [];
        }
        function R0(e) {
            if (e === null) return null;
            do e = e.return;
            while (e && e.tag !== 5 && e.tag !== 27);
            return e || null;
        }
        function $d(e, t, n, a, s) {
            for(var o = t._reactName, f = []; n !== null && n !== a;){
                var g = n, S = g.alternate, w = g.stateNode;
                if (g = g.tag, S !== null && S === a) break;
                g !== 5 && g !== 26 && g !== 27 || w === null || (S = w, s ? (w = la(n, o), w != null && f.unshift(qa(n, w, S))) : s || (w = la(n, o), w != null && f.push(qa(n, w, S)))), n = n.return;
            }
            f.length !== 0 && e.push({
                event: t,
                listeners: f
            });
        }
        var w0 = /\r\n?/g, E0 = /\u0000|\uFFFD/g;
        function Jd(e) {
            return (typeof e == "string" ? e : "" + e).replace(w0, `
`).replace(E0, "");
        }
        function kd(e, t) {
            return t = Jd(t), Jd(e) === t;
        }
        function Ce(e, t, n, a, s, o) {
            switch(n){
                case "children":
                    typeof a == "string" ? t === "body" || t === "textarea" && a === "" || ml(e, a) : (typeof a == "number" || typeof a == "bigint") && t !== "body" && ml(e, "" + a);
                    break;
                case "className":
                    ri(e, "class", a);
                    break;
                case "tabIndex":
                    ri(e, "tabindex", a);
                    break;
                case "dir":
                case "role":
                case "viewBox":
                case "width":
                case "height":
                    ri(e, n, a);
                    break;
                case "style":
                    Jo(e, a, o);
                    break;
                case "data":
                    if (t !== "object") {
                        ri(e, "data", a);
                        break;
                    }
                case "src":
                case "href":
                    if (a === "" && (t !== "a" || n !== "href")) {
                        e.removeAttribute(n);
                        break;
                    }
                    if (a == null || typeof a == "function" || typeof a == "symbol" || typeof a == "boolean") {
                        e.removeAttribute(n);
                        break;
                    }
                    a = ci("" + a), e.setAttribute(n, a);
                    break;
                case "action":
                case "formAction":
                    if (typeof a == "function") {
                        e.setAttribute(n, "javascript:throw new Error('A React form was unexpectedly submitted. If you called form.submit() manually, consider using form.requestSubmit() instead. If you\\'re trying to use event.stopPropagation() in a submit event handler, consider also calling event.preventDefault().')");
                        break;
                    } else typeof o == "function" && (n === "formAction" ? (t !== "input" && Ce(e, t, "name", s.name, s, null), Ce(e, t, "formEncType", s.formEncType, s, null), Ce(e, t, "formMethod", s.formMethod, s, null), Ce(e, t, "formTarget", s.formTarget, s, null)) : (Ce(e, t, "encType", s.encType, s, null), Ce(e, t, "method", s.method, s, null), Ce(e, t, "target", s.target, s, null)));
                    if (a == null || typeof a == "symbol" || typeof a == "boolean") {
                        e.removeAttribute(n);
                        break;
                    }
                    a = ci("" + a), e.setAttribute(n, a);
                    break;
                case "onClick":
                    a != null && (e.onclick = Xt);
                    break;
                case "onScroll":
                    a != null && re("scroll", e);
                    break;
                case "onScrollEnd":
                    a != null && re("scrollend", e);
                    break;
                case "dangerouslySetInnerHTML":
                    if (a != null) {
                        if (typeof a != "object" || !("__html" in a)) throw Error(r(61));
                        if (n = a.__html, n != null) {
                            if (s.children != null) throw Error(r(60));
                            e.innerHTML = n;
                        }
                    }
                    break;
                case "multiple":
                    e.multiple = a && typeof a != "function" && typeof a != "symbol";
                    break;
                case "muted":
                    e.muted = a && typeof a != "function" && typeof a != "symbol";
                    break;
                case "suppressContentEditableWarning":
                case "suppressHydrationWarning":
                case "defaultValue":
                case "defaultChecked":
                case "innerHTML":
                case "ref":
                    break;
                case "autoFocus":
                    break;
                case "xlinkHref":
                    if (a == null || typeof a == "function" || typeof a == "boolean" || typeof a == "symbol") {
                        e.removeAttribute("xlink:href");
                        break;
                    }
                    n = ci("" + a), e.setAttributeNS("http://www.w3.org/1999/xlink", "xlink:href", n);
                    break;
                case "contentEditable":
                case "spellCheck":
                case "draggable":
                case "value":
                case "autoReverse":
                case "externalResourcesRequired":
                case "focusable":
                case "preserveAlpha":
                    a != null && typeof a != "function" && typeof a != "symbol" ? e.setAttribute(n, "" + a) : e.removeAttribute(n);
                    break;
                case "inert":
                case "allowFullScreen":
                case "async":
                case "autoPlay":
                case "controls":
                case "default":
                case "defer":
                case "disabled":
                case "disablePictureInPicture":
                case "disableRemotePlayback":
                case "formNoValidate":
                case "hidden":
                case "loop":
                case "noModule":
                case "noValidate":
                case "open":
                case "playsInline":
                case "readOnly":
                case "required":
                case "reversed":
                case "scoped":
                case "seamless":
                case "itemScope":
                    a && typeof a != "function" && typeof a != "symbol" ? e.setAttribute(n, "") : e.removeAttribute(n);
                    break;
                case "capture":
                case "download":
                    a === !0 ? e.setAttribute(n, "") : a !== !1 && a != null && typeof a != "function" && typeof a != "symbol" ? e.setAttribute(n, a) : e.removeAttribute(n);
                    break;
                case "cols":
                case "rows":
                case "size":
                case "span":
                    a != null && typeof a != "function" && typeof a != "symbol" && !isNaN(a) && 1 <= a ? e.setAttribute(n, a) : e.removeAttribute(n);
                    break;
                case "rowSpan":
                case "start":
                    a == null || typeof a == "function" || typeof a == "symbol" || isNaN(a) ? e.removeAttribute(n) : e.setAttribute(n, a);
                    break;
                case "popover":
                    re("beforetoggle", e), re("toggle", e), si(e, "popover", a);
                    break;
                case "xlinkActuate":
                    Yt(e, "http://www.w3.org/1999/xlink", "xlink:actuate", a);
                    break;
                case "xlinkArcrole":
                    Yt(e, "http://www.w3.org/1999/xlink", "xlink:arcrole", a);
                    break;
                case "xlinkRole":
                    Yt(e, "http://www.w3.org/1999/xlink", "xlink:role", a);
                    break;
                case "xlinkShow":
                    Yt(e, "http://www.w3.org/1999/xlink", "xlink:show", a);
                    break;
                case "xlinkTitle":
                    Yt(e, "http://www.w3.org/1999/xlink", "xlink:title", a);
                    break;
                case "xlinkType":
                    Yt(e, "http://www.w3.org/1999/xlink", "xlink:type", a);
                    break;
                case "xmlBase":
                    Yt(e, "http://www.w3.org/XML/1998/namespace", "xml:base", a);
                    break;
                case "xmlLang":
                    Yt(e, "http://www.w3.org/XML/1998/namespace", "xml:lang", a);
                    break;
                case "xmlSpace":
                    Yt(e, "http://www.w3.org/XML/1998/namespace", "xml:space", a);
                    break;
                case "is":
                    si(e, "is", a);
                    break;
                case "innerText":
                case "textContent":
                    break;
                default:
                    (!(2 < n.length) || n[0] !== "o" && n[0] !== "O" || n[1] !== "n" && n[1] !== "N") && (n = em.get(n) || n, si(e, n, a));
            }
        }
        function Mr(e, t, n, a, s, o) {
            switch(n){
                case "style":
                    Jo(e, a, o);
                    break;
                case "dangerouslySetInnerHTML":
                    if (a != null) {
                        if (typeof a != "object" || !("__html" in a)) throw Error(r(61));
                        if (n = a.__html, n != null) {
                            if (s.children != null) throw Error(r(60));
                            e.innerHTML = n;
                        }
                    }
                    break;
                case "children":
                    typeof a == "string" ? ml(e, a) : (typeof a == "number" || typeof a == "bigint") && ml(e, "" + a);
                    break;
                case "onScroll":
                    a != null && re("scroll", e);
                    break;
                case "onScrollEnd":
                    a != null && re("scrollend", e);
                    break;
                case "onClick":
                    a != null && (e.onclick = Xt);
                    break;
                case "suppressContentEditableWarning":
                case "suppressHydrationWarning":
                case "innerHTML":
                case "ref":
                    break;
                case "innerText":
                case "textContent":
                    break;
                default:
                    if (!Go.hasOwnProperty(n)) e: {
                        if (n[0] === "o" && n[1] === "n" && (s = n.endsWith("Capture"), t = n.slice(2, s ? n.length - 7 : void 0), o = e[nt] || null, o = o != null ? o[n] : null, typeof o == "function" && e.removeEventListener(t, o, s), typeof a == "function")) {
                            typeof o != "function" && o !== null && (n in e ? e[n] = null : e.hasAttribute(n) && e.removeAttribute(n)), e.addEventListener(t, a, s);
                            break e;
                        }
                        n in e ? e[n] = a : a === !0 ? e.setAttribute(n, "") : si(e, n, a);
                    }
            }
        }
        function $e(e, t, n) {
            switch(t){
                case "div":
                case "span":
                case "svg":
                case "path":
                case "a":
                case "g":
                case "p":
                case "li":
                    break;
                case "img":
                    re("error", e), re("load", e);
                    var a = !1, s = !1, o;
                    for(o in n)if (n.hasOwnProperty(o)) {
                        var f = n[o];
                        if (f != null) switch(o){
                            case "src":
                                a = !0;
                                break;
                            case "srcSet":
                                s = !0;
                                break;
                            case "children":
                            case "dangerouslySetInnerHTML":
                                throw Error(r(137, t));
                            default:
                                Ce(e, t, o, f, n, null);
                        }
                    }
                    s && Ce(e, t, "srcSet", n.srcSet, n, null), a && Ce(e, t, "src", n.src, n, null);
                    return;
                case "input":
                    re("invalid", e);
                    var g = o = f = s = null, S = null, w = null;
                    for(a in n)if (n.hasOwnProperty(a)) {
                        var N = n[a];
                        if (N != null) switch(a){
                            case "name":
                                s = N;
                                break;
                            case "type":
                                f = N;
                                break;
                            case "checked":
                                S = N;
                                break;
                            case "defaultChecked":
                                w = N;
                                break;
                            case "value":
                                o = N;
                                break;
                            case "defaultValue":
                                g = N;
                                break;
                            case "children":
                            case "dangerouslySetInnerHTML":
                                if (N != null) throw Error(r(137, t));
                                break;
                            default:
                                Ce(e, t, a, N, n, null);
                        }
                    }
                    Fo(e, o, g, S, w, f, s, !1);
                    return;
                case "select":
                    re("invalid", e), a = f = o = null;
                    for(s in n)if (n.hasOwnProperty(s) && (g = n[s], g != null)) switch(s){
                        case "value":
                            o = g;
                            break;
                        case "defaultValue":
                            f = g;
                            break;
                        case "multiple":
                            a = g;
                        default:
                            Ce(e, t, s, g, n, null);
                    }
                    t = o, n = f, e.multiple = !!a, t != null ? gl(e, !!a, t, !1) : n != null && gl(e, !!a, n, !0);
                    return;
                case "textarea":
                    re("invalid", e), o = s = a = null;
                    for(f in n)if (n.hasOwnProperty(f) && (g = n[f], g != null)) switch(f){
                        case "value":
                            a = g;
                            break;
                        case "defaultValue":
                            s = g;
                            break;
                        case "children":
                            o = g;
                            break;
                        case "dangerouslySetInnerHTML":
                            if (g != null) throw Error(r(91));
                            break;
                        default:
                            Ce(e, t, f, g, n, null);
                    }
                    Ko(e, a, s, o);
                    return;
                case "option":
                    for(S in n)n.hasOwnProperty(S) && (a = n[S], a != null) && (S === "selected" ? e.selected = a && typeof a != "function" && typeof a != "symbol" : Ce(e, t, S, a, n, null));
                    return;
                case "dialog":
                    re("beforetoggle", e), re("toggle", e), re("cancel", e), re("close", e);
                    break;
                case "iframe":
                case "object":
                    re("load", e);
                    break;
                case "video":
                case "audio":
                    for(a = 0; a < Ha.length; a++)re(Ha[a], e);
                    break;
                case "image":
                    re("error", e), re("load", e);
                    break;
                case "details":
                    re("toggle", e);
                    break;
                case "embed":
                case "source":
                case "link":
                    re("error", e), re("load", e);
                case "area":
                case "base":
                case "br":
                case "col":
                case "hr":
                case "keygen":
                case "meta":
                case "param":
                case "track":
                case "wbr":
                case "menuitem":
                    for(w in n)if (n.hasOwnProperty(w) && (a = n[w], a != null)) switch(w){
                        case "children":
                        case "dangerouslySetInnerHTML":
                            throw Error(r(137, t));
                        default:
                            Ce(e, t, w, a, n, null);
                    }
                    return;
                default:
                    if (Bu(t)) {
                        for(N in n)n.hasOwnProperty(N) && (a = n[N], a !== void 0 && Mr(e, t, N, a, n, void 0));
                        return;
                    }
            }
            for(g in n)n.hasOwnProperty(g) && (a = n[g], a != null && Ce(e, t, g, a, n, null));
        }
        function M0(e, t, n, a) {
            switch(t){
                case "div":
                case "span":
                case "svg":
                case "path":
                case "a":
                case "g":
                case "p":
                case "li":
                    break;
                case "input":
                    var s = null, o = null, f = null, g = null, S = null, w = null, N = null;
                    for(O in n){
                        var D = n[O];
                        if (n.hasOwnProperty(O) && D != null) switch(O){
                            case "checked":
                                break;
                            case "value":
                                break;
                            case "defaultValue":
                                S = D;
                            default:
                                a.hasOwnProperty(O) || Ce(e, t, O, null, a, D);
                        }
                    }
                    for(var E in a){
                        var O = a[E];
                        if (D = n[E], a.hasOwnProperty(E) && (O != null || D != null)) switch(E){
                            case "type":
                                o = O;
                                break;
                            case "name":
                                s = O;
                                break;
                            case "checked":
                                w = O;
                                break;
                            case "defaultChecked":
                                N = O;
                                break;
                            case "value":
                                f = O;
                                break;
                            case "defaultValue":
                                g = O;
                                break;
                            case "children":
                            case "dangerouslySetInnerHTML":
                                if (O != null) throw Error(r(137, t));
                                break;
                            default:
                                O !== D && Ce(e, t, E, O, a, D);
                        }
                    }
                    Lu(e, f, g, S, w, N, o, s);
                    return;
                case "select":
                    O = f = g = E = null;
                    for(o in n)if (S = n[o], n.hasOwnProperty(o) && S != null) switch(o){
                        case "value":
                            break;
                        case "multiple":
                            O = S;
                        default:
                            a.hasOwnProperty(o) || Ce(e, t, o, null, a, S);
                    }
                    for(s in a)if (o = a[s], S = n[s], a.hasOwnProperty(s) && (o != null || S != null)) switch(s){
                        case "value":
                            E = o;
                            break;
                        case "defaultValue":
                            g = o;
                            break;
                        case "multiple":
                            f = o;
                        default:
                            o !== S && Ce(e, t, s, o, a, S);
                    }
                    t = g, n = f, a = O, E != null ? gl(e, !!n, E, !1) : !!a != !!n && (t != null ? gl(e, !!n, t, !0) : gl(e, !!n, n ? [] : "", !1));
                    return;
                case "textarea":
                    O = E = null;
                    for(g in n)if (s = n[g], n.hasOwnProperty(g) && s != null && !a.hasOwnProperty(g)) switch(g){
                        case "value":
                            break;
                        case "children":
                            break;
                        default:
                            Ce(e, t, g, null, a, s);
                    }
                    for(f in a)if (s = a[f], o = n[f], a.hasOwnProperty(f) && (s != null || o != null)) switch(f){
                        case "value":
                            E = s;
                            break;
                        case "defaultValue":
                            O = s;
                            break;
                        case "children":
                            break;
                        case "dangerouslySetInnerHTML":
                            if (s != null) throw Error(r(91));
                            break;
                        default:
                            s !== o && Ce(e, t, f, s, a, o);
                    }
                    Zo(e, E, O);
                    return;
                case "option":
                    for(var Q in n)E = n[Q], n.hasOwnProperty(Q) && E != null && !a.hasOwnProperty(Q) && (Q === "selected" ? e.selected = !1 : Ce(e, t, Q, null, a, E));
                    for(S in a)E = a[S], O = n[S], a.hasOwnProperty(S) && E !== O && (E != null || O != null) && (S === "selected" ? e.selected = E && typeof E != "function" && typeof E != "symbol" : Ce(e, t, S, E, a, O));
                    return;
                case "img":
                case "link":
                case "area":
                case "base":
                case "br":
                case "col":
                case "embed":
                case "hr":
                case "keygen":
                case "meta":
                case "param":
                case "source":
                case "track":
                case "wbr":
                case "menuitem":
                    for(var J in n)E = n[J], n.hasOwnProperty(J) && E != null && !a.hasOwnProperty(J) && Ce(e, t, J, null, a, E);
                    for(w in a)if (E = a[w], O = n[w], a.hasOwnProperty(w) && E !== O && (E != null || O != null)) switch(w){
                        case "children":
                        case "dangerouslySetInnerHTML":
                            if (E != null) throw Error(r(137, t));
                            break;
                        default:
                            Ce(e, t, w, E, a, O);
                    }
                    return;
                default:
                    if (Bu(t)) {
                        for(var Re in n)E = n[Re], n.hasOwnProperty(Re) && E !== void 0 && !a.hasOwnProperty(Re) && Mr(e, t, Re, void 0, a, E);
                        for(N in a)E = a[N], O = n[N], !a.hasOwnProperty(N) || E === O || E === void 0 && O === void 0 || Mr(e, t, N, E, a, O);
                        return;
                    }
            }
            for(var C in n)E = n[C], n.hasOwnProperty(C) && E != null && !a.hasOwnProperty(C) && Ce(e, t, C, null, a, E);
            for(D in a)E = a[D], O = n[D], !a.hasOwnProperty(D) || E === O || E == null && O == null || Ce(e, t, D, E, a, O);
        }
        function Wd(e) {
            switch(e){
                case "css":
                case "script":
                case "font":
                case "img":
                case "image":
                case "input":
                case "link":
                    return !0;
                default:
                    return !1;
            }
        }
        function j0() {
            if (typeof performance.getEntriesByType == "function") {
                for(var e = 0, t = 0, n = performance.getEntriesByType("resource"), a = 0; a < n.length; a++){
                    var s = n[a], o = s.transferSize, f = s.initiatorType, g = s.duration;
                    if (o && g && Wd(f)) {
                        for(f = 0, g = s.responseEnd, a += 1; a < n.length; a++){
                            var S = n[a], w = S.startTime;
                            if (w > g) break;
                            var N = S.transferSize, D = S.initiatorType;
                            N && Wd(D) && (S = S.responseEnd, f += N * (S < g ? 1 : (g - w) / (S - w)));
                        }
                        if (--a, t += 8 * (o + f) / (s.duration / 1e3), e++, 10 < e) break;
                    }
                }
                if (0 < e) return t / e / 1e6;
            }
            return navigator.connection && (e = navigator.connection.downlink, typeof e == "number") ? e : 5;
        }
        var jr = null, Or = null;
        function iu(e) {
            return e.nodeType === 9 ? e : e.ownerDocument;
        }
        function Pd(e) {
            switch(e){
                case "http://www.w3.org/2000/svg":
                    return 1;
                case "http://www.w3.org/1998/Math/MathML":
                    return 2;
                default:
                    return 0;
            }
        }
        function Id(e, t) {
            if (e === 0) switch(t){
                case "svg":
                    return 1;
                case "math":
                    return 2;
                default:
                    return 0;
            }
            return e === 1 && t === "foreignObject" ? 0 : e;
        }
        function Tr(e, t) {
            return e === "textarea" || e === "noscript" || typeof t.children == "string" || typeof t.children == "number" || typeof t.children == "bigint" || typeof t.dangerouslySetInnerHTML == "object" && t.dangerouslySetInnerHTML !== null && t.dangerouslySetInnerHTML.__html != null;
        }
        var Nr = null;
        function O0() {
            var e = window.event;
            return e && e.type === "popstate" ? e === Nr ? !1 : (Nr = e, !0) : (Nr = null, !1);
        }
        var eh = typeof setTimeout == "function" ? setTimeout : void 0, T0 = typeof clearTimeout == "function" ? clearTimeout : void 0, th = typeof Promise == "function" ? Promise : void 0, N0 = typeof queueMicrotask == "function" ? queueMicrotask : typeof th < "u" ? function(e) {
            return th.resolve(null).then(e).catch(A0);
        } : eh;
        function A0(e) {
            setTimeout(function() {
                throw e;
            });
        }
        function jn(e) {
            return e === "head";
        }
        function nh(e, t) {
            var n = t, a = 0;
            do {
                var s = n.nextSibling;
                if (e.removeChild(n), s && s.nodeType === 8) if (n = s.data, n === "/$" || n === "/&") {
                    if (a === 0) {
                        e.removeChild(s), Zl(t);
                        return;
                    }
                    a--;
                } else if (n === "$" || n === "$?" || n === "$~" || n === "$!" || n === "&") a++;
                else if (n === "html") Ua(e.ownerDocument.documentElement);
                else if (n === "head") {
                    n = e.ownerDocument.head, Ua(n);
                    for(var o = n.firstChild; o;){
                        var f = o.nextSibling, g = o.nodeName;
                        o[ta] || g === "SCRIPT" || g === "STYLE" || g === "LINK" && o.rel.toLowerCase() === "stylesheet" || n.removeChild(o), o = f;
                    }
                } else n === "body" && Ua(e.ownerDocument.body);
                n = s;
            }while (n);
            Zl(t);
        }
        function lh(e, t) {
            var n = e;
            e = 0;
            do {
                var a = n.nextSibling;
                if (n.nodeType === 1 ? t ? (n._stashedDisplay = n.style.display, n.style.display = "none") : (n.style.display = n._stashedDisplay || "", n.getAttribute("style") === "" && n.removeAttribute("style")) : n.nodeType === 3 && (t ? (n._stashedText = n.nodeValue, n.nodeValue = "") : n.nodeValue = n._stashedText || ""), a && a.nodeType === 8) if (n = a.data, n === "/$") {
                    if (e === 0) break;
                    e--;
                } else n !== "$" && n !== "$?" && n !== "$~" && n !== "$!" || e++;
                n = a;
            }while (n);
        }
        function Ar(e) {
            var t = e.firstChild;
            for(t && t.nodeType === 10 && (t = t.nextSibling); t;){
                var n = t;
                switch(t = t.nextSibling, n.nodeName){
                    case "HTML":
                    case "HEAD":
                    case "BODY":
                        Ar(n), Uu(n);
                        continue;
                    case "SCRIPT":
                    case "STYLE":
                        continue;
                    case "LINK":
                        if (n.rel.toLowerCase() === "stylesheet") continue;
                }
                e.removeChild(n);
            }
        }
        function z0(e, t, n, a) {
            for(; e.nodeType === 1;){
                var s = n;
                if (e.nodeName.toLowerCase() !== t.toLowerCase()) {
                    if (!a && (e.nodeName !== "INPUT" || e.type !== "hidden")) break;
                } else if (a) {
                    if (!e[ta]) switch(t){
                        case "meta":
                            if (!e.hasAttribute("itemprop")) break;
                            return e;
                        case "link":
                            if (o = e.getAttribute("rel"), o === "stylesheet" && e.hasAttribute("data-precedence")) break;
                            if (o !== s.rel || e.getAttribute("href") !== (s.href == null || s.href === "" ? null : s.href) || e.getAttribute("crossorigin") !== (s.crossOrigin == null ? null : s.crossOrigin) || e.getAttribute("title") !== (s.title == null ? null : s.title)) break;
                            return e;
                        case "style":
                            if (e.hasAttribute("data-precedence")) break;
                            return e;
                        case "script":
                            if (o = e.getAttribute("src"), (o !== (s.src == null ? null : s.src) || e.getAttribute("type") !== (s.type == null ? null : s.type) || e.getAttribute("crossorigin") !== (s.crossOrigin == null ? null : s.crossOrigin)) && o && e.hasAttribute("async") && !e.hasAttribute("itemprop")) break;
                            return e;
                        default:
                            return e;
                    }
                } else if (t === "input" && e.type === "hidden") {
                    var o = s.name == null ? null : "" + s.name;
                    if (s.type === "hidden" && e.getAttribute("name") === o) return e;
                } else return e;
                if (e = Ot(e.nextSibling), e === null) break;
            }
            return null;
        }
        function D0(e, t, n) {
            if (t === "") return null;
            for(; e.nodeType !== 3;)if ((e.nodeType !== 1 || e.nodeName !== "INPUT" || e.type !== "hidden") && !n || (e = Ot(e.nextSibling), e === null)) return null;
            return e;
        }
        function ah(e, t) {
            for(; e.nodeType !== 8;)if ((e.nodeType !== 1 || e.nodeName !== "INPUT" || e.type !== "hidden") && !t || (e = Ot(e.nextSibling), e === null)) return null;
            return e;
        }
        function zr(e) {
            return e.data === "$?" || e.data === "$~";
        }
        function Dr(e) {
            return e.data === "$!" || e.data === "$?" && e.ownerDocument.readyState !== "loading";
        }
        function H0(e, t) {
            var n = e.ownerDocument;
            if (e.data === "$~") e._reactRetry = t;
            else if (e.data !== "$?" || n.readyState !== "loading") t();
            else {
                var a = function() {
                    t(), n.removeEventListener("DOMContentLoaded", a);
                };
                n.addEventListener("DOMContentLoaded", a), e._reactRetry = a;
            }
        }
        function Ot(e) {
            for(; e != null; e = e.nextSibling){
                var t = e.nodeType;
                if (t === 1 || t === 3) break;
                if (t === 8) {
                    if (t = e.data, t === "$" || t === "$!" || t === "$?" || t === "$~" || t === "&" || t === "F!" || t === "F") break;
                    if (t === "/$" || t === "/&") return null;
                }
            }
            return e;
        }
        var Hr = null;
        function ih(e) {
            e = e.nextSibling;
            for(var t = 0; e;){
                if (e.nodeType === 8) {
                    var n = e.data;
                    if (n === "/$" || n === "/&") {
                        if (t === 0) return Ot(e.nextSibling);
                        t--;
                    } else n !== "$" && n !== "$!" && n !== "$?" && n !== "$~" && n !== "&" || t++;
                }
                e = e.nextSibling;
            }
            return null;
        }
        function uh(e) {
            e = e.previousSibling;
            for(var t = 0; e;){
                if (e.nodeType === 8) {
                    var n = e.data;
                    if (n === "$" || n === "$!" || n === "$?" || n === "$~" || n === "&") {
                        if (t === 0) return e;
                        t--;
                    } else n !== "/$" && n !== "/&" || t++;
                }
                e = e.previousSibling;
            }
            return null;
        }
        function sh(e, t, n) {
            switch(t = iu(n), e){
                case "html":
                    if (e = t.documentElement, !e) throw Error(r(452));
                    return e;
                case "head":
                    if (e = t.head, !e) throw Error(r(453));
                    return e;
                case "body":
                    if (e = t.body, !e) throw Error(r(454));
                    return e;
                default:
                    throw Error(r(451));
            }
        }
        function Ua(e) {
            for(var t = e.attributes; t.length;)e.removeAttributeNode(t[0]);
            Uu(e);
        }
        var Tt = new Map, rh = new Set;
        function uu(e) {
            return typeof e.getRootNode == "function" ? e.getRootNode() : e.nodeType === 9 ? e : e.ownerDocument;
        }
        var sn = V.d;
        V.d = {
            f: q0,
            r: U0,
            D: V0,
            C: L0,
            L: G0,
            m: B0,
            X: Y0,
            S: Q0,
            M: X0
        };
        function q0() {
            var e = sn.f(), t = Wi();
            return e || t;
        }
        function U0(e) {
            var t = fl(e);
            t !== null && t.tag === 5 && t.type === "form" ? wf(t) : sn.r(e);
        }
        var Yl = typeof document > "u" ? null : document;
        function oh(e, t, n) {
            var a = Yl;
            if (a && typeof t == "string" && t) {
                var s = _t(t);
                s = 'link[rel="' + e + '"][href="' + s + '"]', typeof n == "string" && (s += '[crossorigin="' + n + '"]'), rh.has(s) || (rh.add(s), e = {
                    rel: e,
                    crossOrigin: n,
                    href: t
                }, a.querySelector(s) === null && (t = a.createElement("link"), $e(t, "link", e), Qe(t), a.head.appendChild(t)));
            }
        }
        function V0(e) {
            sn.D(e), oh("dns-prefetch", e, null);
        }
        function L0(e, t) {
            sn.C(e, t), oh("preconnect", e, t);
        }
        function G0(e, t, n) {
            sn.L(e, t, n);
            var a = Yl;
            if (a && e && t) {
                var s = 'link[rel="preload"][as="' + _t(t) + '"]';
                t === "image" && n && n.imageSrcSet ? (s += '[imagesrcset="' + _t(n.imageSrcSet) + '"]', typeof n.imageSizes == "string" && (s += '[imagesizes="' + _t(n.imageSizes) + '"]')) : s += '[href="' + _t(e) + '"]';
                var o = s;
                switch(t){
                    case "style":
                        o = Xl(e);
                        break;
                    case "script":
                        o = Fl(e);
                }
                Tt.has(o) || (e = M({
                    rel: "preload",
                    href: t === "image" && n && n.imageSrcSet ? void 0 : e,
                    as: t
                }, n), Tt.set(o, e), a.querySelector(s) !== null || t === "style" && a.querySelector(Va(o)) || t === "script" && a.querySelector(La(o)) || (t = a.createElement("link"), $e(t, "link", e), Qe(t), a.head.appendChild(t)));
            }
        }
        function B0(e, t) {
            sn.m(e, t);
            var n = Yl;
            if (n && e) {
                var a = t && typeof t.as == "string" ? t.as : "script", s = 'link[rel="modulepreload"][as="' + _t(a) + '"][href="' + _t(e) + '"]', o = s;
                switch(a){
                    case "audioworklet":
                    case "paintworklet":
                    case "serviceworker":
                    case "sharedworker":
                    case "worker":
                    case "script":
                        o = Fl(e);
                }
                if (!Tt.has(o) && (e = M({
                    rel: "modulepreload",
                    href: e
                }, t), Tt.set(o, e), n.querySelector(s) === null)) {
                    switch(a){
                        case "audioworklet":
                        case "paintworklet":
                        case "serviceworker":
                        case "sharedworker":
                        case "worker":
                        case "script":
                            if (n.querySelector(La(o))) return;
                    }
                    a = n.createElement("link"), $e(a, "link", e), Qe(a), n.head.appendChild(a);
                }
            }
        }
        function Q0(e, t, n) {
            sn.S(e, t, n);
            var a = Yl;
            if (a && e) {
                var s = dl(a).hoistableStyles, o = Xl(e);
                t = t || "default";
                var f = s.get(o);
                if (!f) {
                    var g = {
                        loading: 0,
                        preload: null
                    };
                    if (f = a.querySelector(Va(o))) g.loading = 5;
                    else {
                        e = M({
                            rel: "stylesheet",
                            href: e,
                            "data-precedence": t
                        }, n), (n = Tt.get(o)) && qr(e, n);
                        var S = f = a.createElement("link");
                        Qe(S), $e(S, "link", e), S._p = new Promise(function(w, N) {
                            S.onload = w, S.onerror = N;
                        }), S.addEventListener("load", function() {
                            g.loading |= 1;
                        }), S.addEventListener("error", function() {
                            g.loading |= 2;
                        }), g.loading |= 4, su(f, t, a);
                    }
                    f = {
                        type: "stylesheet",
                        instance: f,
                        count: 1,
                        state: g
                    }, s.set(o, f);
                }
            }
        }
        function Y0(e, t) {
            sn.X(e, t);
            var n = Yl;
            if (n && e) {
                var a = dl(n).hoistableScripts, s = Fl(e), o = a.get(s);
                o || (o = n.querySelector(La(s)), o || (e = M({
                    src: e,
                    async: !0
                }, t), (t = Tt.get(s)) && Ur(e, t), o = n.createElement("script"), Qe(o), $e(o, "link", e), n.head.appendChild(o)), o = {
                    type: "script",
                    instance: o,
                    count: 1,
                    state: null
                }, a.set(s, o));
            }
        }
        function X0(e, t) {
            sn.M(e, t);
            var n = Yl;
            if (n && e) {
                var a = dl(n).hoistableScripts, s = Fl(e), o = a.get(s);
                o || (o = n.querySelector(La(s)), o || (e = M({
                    src: e,
                    async: !0,
                    type: "module"
                }, t), (t = Tt.get(s)) && Ur(e, t), o = n.createElement("script"), Qe(o), $e(o, "link", e), n.head.appendChild(o)), o = {
                    type: "script",
                    instance: o,
                    count: 1,
                    state: null
                }, a.set(s, o));
            }
        }
        function ch(e, t, n, a) {
            var s = (s = ue.current) ? uu(s) : null;
            if (!s) throw Error(r(446));
            switch(e){
                case "meta":
                case "title":
                    return null;
                case "style":
                    return typeof n.precedence == "string" && typeof n.href == "string" ? (t = Xl(n.href), n = dl(s).hoistableStyles, a = n.get(t), a || (a = {
                        type: "style",
                        instance: null,
                        count: 0,
                        state: null
                    }, n.set(t, a)), a) : {
                        type: "void",
                        instance: null,
                        count: 0,
                        state: null
                    };
                case "link":
                    if (n.rel === "stylesheet" && typeof n.href == "string" && typeof n.precedence == "string") {
                        e = Xl(n.href);
                        var o = dl(s).hoistableStyles, f = o.get(e);
                        if (f || (s = s.ownerDocument || s, f = {
                            type: "stylesheet",
                            instance: null,
                            count: 0,
                            state: {
                                loading: 0,
                                preload: null
                            }
                        }, o.set(e, f), (o = s.querySelector(Va(e))) && !o._p && (f.instance = o, f.state.loading = 5), Tt.has(e) || (n = {
                            rel: "preload",
                            as: "style",
                            href: n.href,
                            crossOrigin: n.crossOrigin,
                            integrity: n.integrity,
                            media: n.media,
                            hrefLang: n.hrefLang,
                            referrerPolicy: n.referrerPolicy
                        }, Tt.set(e, n), o || F0(s, e, n, f.state))), t && a === null) throw Error(r(528, ""));
                        return f;
                    }
                    if (t && a !== null) throw Error(r(529, ""));
                    return null;
                case "script":
                    return t = n.async, n = n.src, typeof n == "string" && t && typeof t != "function" && typeof t != "symbol" ? (t = Fl(n), n = dl(s).hoistableScripts, a = n.get(t), a || (a = {
                        type: "script",
                        instance: null,
                        count: 0,
                        state: null
                    }, n.set(t, a)), a) : {
                        type: "void",
                        instance: null,
                        count: 0,
                        state: null
                    };
                default:
                    throw Error(r(444, e));
            }
        }
        function Xl(e) {
            return 'href="' + _t(e) + '"';
        }
        function Va(e) {
            return 'link[rel="stylesheet"][' + e + "]";
        }
        function fh(e) {
            return M({}, e, {
                "data-precedence": e.precedence,
                precedence: null
            });
        }
        function F0(e, t, n, a) {
            e.querySelector('link[rel="preload"][as="style"][' + t + "]") ? a.loading = 1 : (t = e.createElement("link"), a.preload = t, t.addEventListener("load", function() {
                return a.loading |= 1;
            }), t.addEventListener("error", function() {
                return a.loading |= 2;
            }), $e(t, "link", n), Qe(t), e.head.appendChild(t));
        }
        function Fl(e) {
            return '[src="' + _t(e) + '"]';
        }
        function La(e) {
            return "script[async]" + e;
        }
        function dh(e, t, n) {
            if (t.count++, t.instance === null) switch(t.type){
                case "style":
                    var a = e.querySelector('style[data-href~="' + _t(n.href) + '"]');
                    if (a) return t.instance = a, Qe(a), a;
                    var s = M({}, n, {
                        "data-href": n.href,
                        "data-precedence": n.precedence,
                        href: null,
                        precedence: null
                    });
                    return a = (e.ownerDocument || e).createElement("style"), Qe(a), $e(a, "style", s), su(a, n.precedence, e), t.instance = a;
                case "stylesheet":
                    s = Xl(n.href);
                    var o = e.querySelector(Va(s));
                    if (o) return t.state.loading |= 4, t.instance = o, Qe(o), o;
                    a = fh(n), (s = Tt.get(s)) && qr(a, s), o = (e.ownerDocument || e).createElement("link"), Qe(o);
                    var f = o;
                    return f._p = new Promise(function(g, S) {
                        f.onload = g, f.onerror = S;
                    }), $e(o, "link", a), t.state.loading |= 4, su(o, n.precedence, e), t.instance = o;
                case "script":
                    return o = Fl(n.src), (s = e.querySelector(La(o))) ? (t.instance = s, Qe(s), s) : (a = n, (s = Tt.get(o)) && (a = M({}, n), Ur(a, s)), e = e.ownerDocument || e, s = e.createElement("script"), Qe(s), $e(s, "link", a), e.head.appendChild(s), t.instance = s);
                case "void":
                    return null;
                default:
                    throw Error(r(443, t.type));
            }
            else t.type === "stylesheet" && (t.state.loading & 4) === 0 && (a = t.instance, t.state.loading |= 4, su(a, n.precedence, e));
            return t.instance;
        }
        function su(e, t, n) {
            for(var a = n.querySelectorAll('link[rel="stylesheet"][data-precedence],style[data-precedence]'), s = a.length ? a[a.length - 1] : null, o = s, f = 0; f < a.length; f++){
                var g = a[f];
                if (g.dataset.precedence === t) o = g;
                else if (o !== s) break;
            }
            o ? o.parentNode.insertBefore(e, o.nextSibling) : (t = n.nodeType === 9 ? n.head : n, t.insertBefore(e, t.firstChild));
        }
        function qr(e, t) {
            e.crossOrigin == null && (e.crossOrigin = t.crossOrigin), e.referrerPolicy == null && (e.referrerPolicy = t.referrerPolicy), e.title == null && (e.title = t.title);
        }
        function Ur(e, t) {
            e.crossOrigin == null && (e.crossOrigin = t.crossOrigin), e.referrerPolicy == null && (e.referrerPolicy = t.referrerPolicy), e.integrity == null && (e.integrity = t.integrity);
        }
        var ru = null;
        function hh(e, t, n) {
            if (ru === null) {
                var a = new Map, s = ru = new Map;
                s.set(n, a);
            } else s = ru, a = s.get(n), a || (a = new Map, s.set(n, a));
            if (a.has(e)) return a;
            for(a.set(e, null), n = n.getElementsByTagName(e), s = 0; s < n.length; s++){
                var o = n[s];
                if (!(o[ta] || o[Xe] || e === "link" && o.getAttribute("rel") === "stylesheet") && o.namespaceURI !== "http://www.w3.org/2000/svg") {
                    var f = o.getAttribute(t) || "";
                    f = e + f;
                    var g = a.get(f);
                    g ? g.push(o) : a.set(f, [
                        o
                    ]);
                }
            }
            return a;
        }
        function gh(e, t, n) {
            e = e.ownerDocument || e, e.head.insertBefore(n, t === "title" ? e.querySelector("head > title") : null);
        }
        function Z0(e, t, n) {
            if (n === 1 || t.itemProp != null) return !1;
            switch(e){
                case "meta":
                case "title":
                    return !0;
                case "style":
                    if (typeof t.precedence != "string" || typeof t.href != "string" || t.href === "") break;
                    return !0;
                case "link":
                    if (typeof t.rel != "string" || typeof t.href != "string" || t.href === "" || t.onLoad || t.onError) break;
                    return t.rel === "stylesheet" ? (e = t.disabled, typeof t.precedence == "string" && e == null) : !0;
                case "script":
                    if (t.async && typeof t.async != "function" && typeof t.async != "symbol" && !t.onLoad && !t.onError && t.src && typeof t.src == "string") return !0;
            }
            return !1;
        }
        function mh(e) {
            return !(e.type === "stylesheet" && (e.state.loading & 3) === 0);
        }
        function K0(e, t, n, a) {
            if (n.type === "stylesheet" && (typeof a.media != "string" || matchMedia(a.media).matches !== !1) && (n.state.loading & 4) === 0) {
                if (n.instance === null) {
                    var s = Xl(a.href), o = t.querySelector(Va(s));
                    if (o) {
                        t = o._p, t !== null && typeof t == "object" && typeof t.then == "function" && (e.count++, e = ou.bind(e), t.then(e, e)), n.state.loading |= 4, n.instance = o, Qe(o);
                        return;
                    }
                    o = t.ownerDocument || t, a = fh(a), (s = Tt.get(s)) && qr(a, s), o = o.createElement("link"), Qe(o);
                    var f = o;
                    f._p = new Promise(function(g, S) {
                        f.onload = g, f.onerror = S;
                    }), $e(o, "link", a), n.instance = o;
                }
                e.stylesheets === null && (e.stylesheets = new Map), e.stylesheets.set(n, t), (t = n.state.preload) && (n.state.loading & 3) === 0 && (e.count++, n = ou.bind(e), t.addEventListener("load", n), t.addEventListener("error", n));
            }
        }
        var Vr = 0;
        function $0(e, t) {
            return e.stylesheets && e.count === 0 && fu(e, e.stylesheets), 0 < e.count || 0 < e.imgCount ? function(n) {
                var a = setTimeout(function() {
                    if (e.stylesheets && fu(e, e.stylesheets), e.unsuspend) {
                        var o = e.unsuspend;
                        e.unsuspend = null, o();
                    }
                }, 6e4 + t);
                0 < e.imgBytes && Vr === 0 && (Vr = 62500 * j0());
                var s = setTimeout(function() {
                    if (e.waitingForImages = !1, e.count === 0 && (e.stylesheets && fu(e, e.stylesheets), e.unsuspend)) {
                        var o = e.unsuspend;
                        e.unsuspend = null, o();
                    }
                }, (e.imgBytes > Vr ? 50 : 800) + t);
                return e.unsuspend = n, function() {
                    e.unsuspend = null, clearTimeout(a), clearTimeout(s);
                };
            } : null;
        }
        function ou() {
            if (this.count--, this.count === 0 && (this.imgCount === 0 || !this.waitingForImages)) {
                if (this.stylesheets) fu(this, this.stylesheets);
                else if (this.unsuspend) {
                    var e = this.unsuspend;
                    this.unsuspend = null, e();
                }
            }
        }
        var cu = null;
        function fu(e, t) {
            e.stylesheets = null, e.unsuspend !== null && (e.count++, cu = new Map, t.forEach(J0, e), cu = null, ou.call(e));
        }
        function J0(e, t) {
            if (!(t.state.loading & 4)) {
                var n = cu.get(e);
                if (n) var a = n.get(null);
                else {
                    n = new Map, cu.set(e, n);
                    for(var s = e.querySelectorAll("link[data-precedence],style[data-precedence]"), o = 0; o < s.length; o++){
                        var f = s[o];
                        (f.nodeName === "LINK" || f.getAttribute("media") !== "not all") && (n.set(f.dataset.precedence, f), a = f);
                    }
                    a && n.set(null, a);
                }
                s = t.instance, f = s.getAttribute("data-precedence"), o = n.get(f) || a, o === a && n.set(null, s), n.set(f, s), this.count++, a = ou.bind(this), s.addEventListener("load", a), s.addEventListener("error", a), o ? o.parentNode.insertBefore(s, o.nextSibling) : (e = e.nodeType === 9 ? e.head : e, e.insertBefore(s, e.firstChild)), t.state.loading |= 4;
            }
        }
        var Ga = {
            $$typeof: Z,
            Provider: null,
            Consumer: null,
            _currentValue: k,
            _currentValue2: k,
            _threadCount: 0
        };
        function k0(e, t, n, a, s, o, f, g, S) {
            this.tag = 1, this.containerInfo = e, this.pingCache = this.current = this.pendingChildren = null, this.timeoutHandle = -1, this.callbackNode = this.next = this.pendingContext = this.context = this.cancelPendingCommit = null, this.callbackPriority = 0, this.expirationTimes = zu(-1), this.entangledLanes = this.shellSuspendCounter = this.errorRecoveryDisabledLanes = this.expiredLanes = this.warmLanes = this.pingedLanes = this.suspendedLanes = this.pendingLanes = 0, this.entanglements = zu(0), this.hiddenUpdates = zu(null), this.identifierPrefix = a, this.onUncaughtError = s, this.onCaughtError = o, this.onRecoverableError = f, this.pooledCache = null, this.pooledCacheLanes = 0, this.formState = S, this.incompleteTransitions = new Map;
        }
        function yh(e, t, n, a, s, o, f, g, S, w, N, D) {
            return e = new k0(e, t, n, f, S, w, N, D, g), t = 1, o === !0 && (t |= 24), o = gt(3, null, null, t), e.current = o, o.stateNode = e, t = vs(), t.refCount++, e.pooledCache = t, t.refCount++, o.memoizedState = {
                element: a,
                isDehydrated: n,
                cache: t
            }, xs(o), e;
        }
        function vh(e) {
            return e ? (e = _l, e) : _l;
        }
        function ph(e, t, n, a, s, o) {
            s = vh(s), a.context === null ? a.context = s : a.pendingContext = s, a = vn(t), a.payload = {
                element: n
            }, o = o === void 0 ? null : o, o !== null && (a.callback = o), n = pn(e, a, t), n !== null && (rt(n, e, t), pa(n, e, t));
        }
        function Sh(e, t) {
            if (e = e.memoizedState, e !== null && e.dehydrated !== null) {
                var n = e.retryLane;
                e.retryLane = n !== 0 && n < t ? n : t;
            }
        }
        function Lr(e, t) {
            Sh(e, t), (e = e.alternate) && Sh(e, t);
        }
        function bh(e) {
            if (e.tag === 13 || e.tag === 31) {
                var t = Zn(e, 67108864);
                t !== null && rt(t, e, 67108864), Lr(e, 67108864);
            }
        }
        function xh(e) {
            if (e.tag === 13 || e.tag === 31) {
                var t = St();
                t = Du(t);
                var n = Zn(e, t);
                n !== null && rt(n, e, t), Lr(e, t);
            }
        }
        var du = !0;
        function W0(e, t, n, a) {
            var s = A.T;
            A.T = null;
            var o = V.p;
            try {
                V.p = 2, Gr(e, t, n, a);
            } finally{
                V.p = o, A.T = s;
            }
        }
        function P0(e, t, n, a) {
            var s = A.T;
            A.T = null;
            var o = V.p;
            try {
                V.p = 8, Gr(e, t, n, a);
            } finally{
                V.p = o, A.T = s;
            }
        }
        function Gr(e, t, n, a) {
            if (du) {
                var s = Br(a);
                if (s === null) Er(e, t, a, hu, n), Ch(e, a);
                else if (ey(s, e, t, n, a)) a.stopPropagation();
                else if (Ch(e, a), t & 4 && -1 < I0.indexOf(e)) {
                    for(; s !== null;){
                        var o = fl(s);
                        if (o !== null) switch(o.tag){
                            case 3:
                                if (o = o.stateNode, o.current.memoizedState.isDehydrated) {
                                    var f = Bn(o.pendingLanes);
                                    if (f !== 0) {
                                        var g = o;
                                        for(g.pendingLanes |= 2, g.entangledLanes |= 2; f;){
                                            var S = 1 << 31 - dt(f);
                                            g.entanglements[1] |= S, f &= ~S;
                                        }
                                        Bt(o), (ye & 6) === 0 && (Ji = ct() + 500, Da(0));
                                    }
                                }
                                break;
                            case 31:
                            case 13:
                                g = Zn(o, 2), g !== null && rt(g, o, 2), Wi(), Lr(o, 2);
                        }
                        if (o = Br(a), o === null && Er(e, t, a, hu, n), o === s) break;
                        s = o;
                    }
                    s !== null && a.stopPropagation();
                } else Er(e, t, a, null, n);
            }
        }
        function Br(e) {
            return e = Yu(e), Qr(e);
        }
        var hu = null;
        function Qr(e) {
            if (hu = null, e = cl(e), e !== null) {
                var t = d(e);
                if (t === null) e = null;
                else {
                    var n = t.tag;
                    if (n === 13) {
                        if (e = m(t), e !== null) return e;
                        e = null;
                    } else if (n === 31) {
                        if (e = v(t), e !== null) return e;
                        e = null;
                    } else if (n === 3) {
                        if (t.stateNode.current.memoizedState.isDehydrated) return t.tag === 3 ? t.stateNode.containerInfo : null;
                        e = null;
                    } else t !== e && (e = null);
                }
            }
            return hu = e, null;
        }
        function _h(e) {
            switch(e){
                case "beforetoggle":
                case "cancel":
                case "click":
                case "close":
                case "contextmenu":
                case "copy":
                case "cut":
                case "auxclick":
                case "dblclick":
                case "dragend":
                case "dragstart":
                case "drop":
                case "focusin":
                case "focusout":
                case "input":
                case "invalid":
                case "keydown":
                case "keypress":
                case "keyup":
                case "mousedown":
                case "mouseup":
                case "paste":
                case "pause":
                case "play":
                case "pointercancel":
                case "pointerdown":
                case "pointerup":
                case "ratechange":
                case "reset":
                case "resize":
                case "seeked":
                case "submit":
                case "toggle":
                case "touchcancel":
                case "touchend":
                case "touchstart":
                case "volumechange":
                case "change":
                case "selectionchange":
                case "textInput":
                case "compositionstart":
                case "compositionend":
                case "compositionupdate":
                case "beforeblur":
                case "afterblur":
                case "beforeinput":
                case "blur":
                case "fullscreenchange":
                case "focus":
                case "hashchange":
                case "popstate":
                case "select":
                case "selectstart":
                    return 2;
                case "drag":
                case "dragenter":
                case "dragexit":
                case "dragleave":
                case "dragover":
                case "mousemove":
                case "mouseout":
                case "mouseover":
                case "pointermove":
                case "pointerout":
                case "pointerover":
                case "scroll":
                case "touchmove":
                case "wheel":
                case "mouseenter":
                case "mouseleave":
                case "pointerenter":
                case "pointerleave":
                    return 8;
                case "message":
                    switch(Vg()){
                        case Oo:
                            return 2;
                        case To:
                            return 8;
                        case ni:
                        case Lg:
                            return 32;
                        case No:
                            return 268435456;
                        default:
                            return 32;
                    }
                default:
                    return 32;
            }
        }
        var Yr = !1, On = null, Tn = null, Nn = null, Ba = new Map, Qa = new Map, An = [], I0 = "mousedown mouseup touchcancel touchend touchstart auxclick dblclick pointercancel pointerdown pointerup dragend dragstart drop compositionend compositionstart keydown keypress keyup input textInput copy cut paste click change contextmenu reset".split(" ");
        function Ch(e, t) {
            switch(e){
                case "focusin":
                case "focusout":
                    On = null;
                    break;
                case "dragenter":
                case "dragleave":
                    Tn = null;
                    break;
                case "mouseover":
                case "mouseout":
                    Nn = null;
                    break;
                case "pointerover":
                case "pointerout":
                    Ba.delete(t.pointerId);
                    break;
                case "gotpointercapture":
                case "lostpointercapture":
                    Qa.delete(t.pointerId);
            }
        }
        function Ya(e, t, n, a, s, o) {
            return e === null || e.nativeEvent !== o ? (e = {
                blockedOn: t,
                domEventName: n,
                eventSystemFlags: a,
                nativeEvent: o,
                targetContainers: [
                    s
                ]
            }, t !== null && (t = fl(t), t !== null && bh(t)), e) : (e.eventSystemFlags |= a, t = e.targetContainers, s !== null && t.indexOf(s) === -1 && t.push(s), e);
        }
        function ey(e, t, n, a, s) {
            switch(t){
                case "focusin":
                    return On = Ya(On, e, t, n, a, s), !0;
                case "dragenter":
                    return Tn = Ya(Tn, e, t, n, a, s), !0;
                case "mouseover":
                    return Nn = Ya(Nn, e, t, n, a, s), !0;
                case "pointerover":
                    var o = s.pointerId;
                    return Ba.set(o, Ya(Ba.get(o) || null, e, t, n, a, s)), !0;
                case "gotpointercapture":
                    return o = s.pointerId, Qa.set(o, Ya(Qa.get(o) || null, e, t, n, a, s)), !0;
            }
            return !1;
        }
        function Rh(e) {
            var t = cl(e.target);
            if (t !== null) {
                var n = d(t);
                if (n !== null) {
                    if (t = n.tag, t === 13) {
                        if (t = m(n), t !== null) {
                            e.blockedOn = t, Uo(e.priority, function() {
                                xh(n);
                            });
                            return;
                        }
                    } else if (t === 31) {
                        if (t = v(n), t !== null) {
                            e.blockedOn = t, Uo(e.priority, function() {
                                xh(n);
                            });
                            return;
                        }
                    } else if (t === 3 && n.stateNode.current.memoizedState.isDehydrated) {
                        e.blockedOn = n.tag === 3 ? n.stateNode.containerInfo : null;
                        return;
                    }
                }
            }
            e.blockedOn = null;
        }
        function gu(e) {
            if (e.blockedOn !== null) return !1;
            for(var t = e.targetContainers; 0 < t.length;){
                var n = Br(e.nativeEvent);
                if (n === null) {
                    n = e.nativeEvent;
                    var a = new n.constructor(n.type, n);
                    Qu = a, n.target.dispatchEvent(a), Qu = null;
                } else return t = fl(n), t !== null && bh(t), e.blockedOn = n, !1;
                t.shift();
            }
            return !0;
        }
        function wh(e, t, n) {
            gu(e) && n.delete(t);
        }
        function ty() {
            Yr = !1, On !== null && gu(On) && (On = null), Tn !== null && gu(Tn) && (Tn = null), Nn !== null && gu(Nn) && (Nn = null), Ba.forEach(wh), Qa.forEach(wh);
        }
        function mu(e, t) {
            e.blockedOn === t && (e.blockedOn = null, Yr || (Yr = !0, l.unstable_scheduleCallback(l.unstable_NormalPriority, ty)));
        }
        var yu = null;
        function Eh(e) {
            yu !== e && (yu = e, l.unstable_scheduleCallback(l.unstable_NormalPriority, function() {
                yu === e && (yu = null);
                for(var t = 0; t < e.length; t += 3){
                    var n = e[t], a = e[t + 1], s = e[t + 2];
                    if (typeof a != "function") {
                        if (Qr(a || n) === null) continue;
                        break;
                    }
                    var o = fl(n);
                    o !== null && (e.splice(t, 3), t -= 3, Bs(o, {
                        pending: !0,
                        data: s,
                        method: n.method,
                        action: a
                    }, a, s));
                }
            }));
        }
        function Zl(e) {
            function t(S) {
                return mu(S, e);
            }
            On !== null && mu(On, e), Tn !== null && mu(Tn, e), Nn !== null && mu(Nn, e), Ba.forEach(t), Qa.forEach(t);
            for(var n = 0; n < An.length; n++){
                var a = An[n];
                a.blockedOn === e && (a.blockedOn = null);
            }
            for(; 0 < An.length && (n = An[0], n.blockedOn === null);)Rh(n), n.blockedOn === null && An.shift();
            if (n = (e.ownerDocument || e).$$reactFormReplay, n != null) for(a = 0; a < n.length; a += 3){
                var s = n[a], o = n[a + 1], f = s[nt] || null;
                if (typeof o == "function") f || Eh(n);
                else if (f) {
                    var g = null;
                    if (o && o.hasAttribute("formAction")) {
                        if (s = o, f = o[nt] || null) g = f.formAction;
                        else if (Qr(s) !== null) continue;
                    } else g = f.action;
                    typeof g == "function" ? n[a + 1] = g : (n.splice(a, 3), a -= 3), Eh(n);
                }
            }
        }
        function Mh() {
            function e(o) {
                o.canIntercept && o.info === "react-transition" && o.intercept({
                    handler: function() {
                        return new Promise(function(f) {
                            return s = f;
                        });
                    },
                    focusReset: "manual",
                    scroll: "manual"
                });
            }
            function t() {
                s !== null && (s(), s = null), a || setTimeout(n, 20);
            }
            function n() {
                if (!a && !navigation.transition) {
                    var o = navigation.currentEntry;
                    o && o.url != null && navigation.navigate(o.url, {
                        state: o.getState(),
                        info: "react-transition",
                        history: "replace"
                    });
                }
            }
            if (typeof navigation == "object") {
                var a = !1, s = null;
                return navigation.addEventListener("navigate", e), navigation.addEventListener("navigatesuccess", t), navigation.addEventListener("navigateerror", t), setTimeout(n, 100), function() {
                    a = !0, navigation.removeEventListener("navigate", e), navigation.removeEventListener("navigatesuccess", t), navigation.removeEventListener("navigateerror", t), s !== null && (s(), s = null);
                };
            }
        }
        function Xr(e) {
            this._internalRoot = e;
        }
        vu.prototype.render = Xr.prototype.render = function(e) {
            var t = this._internalRoot;
            if (t === null) throw Error(r(409));
            var n = t.current, a = St();
            ph(n, a, e, t, null, null);
        }, vu.prototype.unmount = Xr.prototype.unmount = function() {
            var e = this._internalRoot;
            if (e !== null) {
                this._internalRoot = null;
                var t = e.containerInfo;
                ph(e.current, 2, null, e, null, null), Wi(), t[ol] = null;
            }
        };
        function vu(e) {
            this._internalRoot = e;
        }
        vu.prototype.unstable_scheduleHydration = function(e) {
            if (e) {
                var t = qo();
                e = {
                    blockedOn: null,
                    target: e,
                    priority: t
                };
                for(var n = 0; n < An.length && t !== 0 && t < An[n].priority; n++);
                An.splice(n, 0, e), n === 0 && Rh(e);
            }
        };
        var jh = i.version;
        if (jh !== "19.2.1") throw Error(r(527, jh, "19.2.1"));
        V.findDOMNode = function(e) {
            var t = e._reactInternals;
            if (t === void 0) throw typeof e.render == "function" ? Error(r(188)) : (e = Object.keys(e).join(","), Error(r(268, e)));
            return e = p(t), e = e !== null ? _(e) : null, e = e === null ? null : e.stateNode, e;
        };
        var ny = {
            bundleType: 0,
            version: "19.2.1",
            rendererPackageName: "react-dom",
            currentDispatcherRef: A,
            reconcilerVersion: "19.2.1"
        };
        if (typeof __REACT_DEVTOOLS_GLOBAL_HOOK__ < "u") {
            var pu = __REACT_DEVTOOLS_GLOBAL_HOOK__;
            if (!pu.isDisabled && pu.supportsFiber) try {
                Pl = pu.inject(ny), ft = pu;
            } catch  {}
        }
        return Fa.createRoot = function(e, t) {
            if (!c(e)) throw Error(r(299));
            var n = !1, a = "", s = Hf, o = qf, f = Uf;
            return t != null && (t.unstable_strictMode === !0 && (n = !0), t.identifierPrefix !== void 0 && (a = t.identifierPrefix), t.onUncaughtError !== void 0 && (s = t.onUncaughtError), t.onCaughtError !== void 0 && (o = t.onCaughtError), t.onRecoverableError !== void 0 && (f = t.onRecoverableError)), t = yh(e, 1, !1, null, null, n, a, null, s, o, f, Mh), e[ol] = t.current, wr(e), new Xr(t);
        }, Fa.hydrateRoot = function(e, t, n) {
            if (!c(e)) throw Error(r(299));
            var a = !1, s = "", o = Hf, f = qf, g = Uf, S = null;
            return n != null && (n.unstable_strictMode === !0 && (a = !0), n.identifierPrefix !== void 0 && (s = n.identifierPrefix), n.onUncaughtError !== void 0 && (o = n.onUncaughtError), n.onCaughtError !== void 0 && (f = n.onCaughtError), n.onRecoverableError !== void 0 && (g = n.onRecoverableError), n.formState !== void 0 && (S = n.formState)), t = yh(e, 1, !0, t, n ?? null, a, s, S, o, f, g, Mh), t.context = vh(null), n = t.current, a = St(), a = Du(a), s = vn(a), s.callback = null, pn(n, s, a), n = a, t.current.lanes = n, ea(t, n), Bt(t), e[ol] = t.current, wr(e), new vu(t);
        }, Fa.version = "19.2.1", Fa;
    }
    var Vh;
    function dy() {
        if (Vh) return Kr.exports;
        Vh = 1;
        function l() {
            if (!(typeof __REACT_DEVTOOLS_GLOBAL_HOOK__ > "u" || typeof __REACT_DEVTOOLS_GLOBAL_HOOK__.checkDCE != "function")) try {
                __REACT_DEVTOOLS_GLOBAL_HOOK__.checkDCE(l);
            } catch (i) {
                console.error(i);
            }
        }
        return l(), Kr.exports = fy(), Kr.exports;
    }
    var hy = dy(), ei = class {
        constructor(){
            this.listeners = new Set, this.subscribe = this.subscribe.bind(this);
        }
        subscribe(l) {
            return this.listeners.add(l), this.onSubscribe(), ()=>{
                this.listeners.delete(l), this.onUnsubscribe();
            };
        }
        hasListeners() {
            return this.listeners.size > 0;
        }
        onSubscribe() {}
        onUnsubscribe() {}
    }, gy = {
        setTimeout: (l, i)=>setTimeout(l, i),
        clearTimeout: (l)=>clearTimeout(l),
        setInterval: (l, i)=>setInterval(l, i),
        clearInterval: (l)=>clearInterval(l)
    }, my = class {
        #e = gy;
        #t = !1;
        setTimeoutProvider(l) {
            this.#e = l;
        }
        setTimeout(l, i) {
            return this.#e.setTimeout(l, i);
        }
        clearTimeout(l) {
            this.#e.clearTimeout(l);
        }
        setInterval(l, i) {
            return this.#e.setInterval(l, i);
        }
        clearInterval(l) {
            this.#e.clearInterval(l);
        }
    }, ul = new my;
    function yy(l) {
        setTimeout(l, 0);
    }
    var sl = typeof window > "u" || "Deno" in globalThis;
    function ot() {}
    function vy(l, i) {
        return typeof l == "function" ? l(i) : l;
    }
    function uo(l) {
        return typeof l == "number" && l >= 0 && l !== 1 / 0;
    }
    function hg(l, i) {
        return Math.max(l + (i || 0) - Date.now(), 0);
    }
    function qn(l, i) {
        return typeof l == "function" ? l(i) : l;
    }
    function Nt(l, i) {
        return typeof l == "function" ? l(i) : l;
    }
    function Lh(l, i) {
        const { type: u = "all", exact: r, fetchStatus: c, predicate: d, queryKey: m, stale: v } = l;
        if (m) {
            if (r) {
                if (i.queryHash !== bo(m, i.options)) return !1;
            } else if (!Pa(i.queryKey, m)) return !1;
        }
        if (u !== "all") {
            const y = i.isActive();
            if (u === "active" && !y || u === "inactive" && y) return !1;
        }
        return !(typeof v == "boolean" && i.isStale() !== v || c && c !== i.state.fetchStatus || d && !d(i));
    }
    function Gh(l, i) {
        const { exact: u, status: r, predicate: c, mutationKey: d } = l;
        if (d) {
            if (!i.options.mutationKey) return !1;
            if (u) {
                if (Wa(i.options.mutationKey) !== Wa(d)) return !1;
            } else if (!Pa(i.options.mutationKey, d)) return !1;
        }
        return !(r && i.state.status !== r || c && !c(i));
    }
    function bo(l, i) {
        return (i?.queryKeyHashFn || Wa)(l);
    }
    function Wa(l) {
        return JSON.stringify(l, (i, u)=>ro(u) ? Object.keys(u).sort().reduce((r, c)=>(r[c] = u[c], r), {}) : u);
    }
    function Pa(l, i) {
        return l === i ? !0 : typeof l != typeof i ? !1 : l && i && typeof l == "object" && typeof i == "object" ? Object.keys(i).every((u)=>Pa(l[u], i[u])) : !1;
    }
    var py = Object.prototype.hasOwnProperty;
    function gg(l, i) {
        if (l === i) return l;
        const u = Bh(l) && Bh(i);
        if (!u && !(ro(l) && ro(i))) return i;
        const c = (u ? l : Object.keys(l)).length, d = u ? i : Object.keys(i), m = d.length, v = u ? new Array(m) : {};
        let y = 0;
        for(let p = 0; p < m; p++){
            const _ = u ? p : d[p], M = l[_], j = i[_];
            if (M === j) {
                v[_] = M, (u ? p < c : py.call(l, _)) && y++;
                continue;
            }
            if (M === null || j === null || typeof M != "object" || typeof j != "object") {
                v[_] = j;
                continue;
            }
            const T = gg(M, j);
            v[_] = T, T === M && y++;
        }
        return c === m && y === c ? l : v;
    }
    function so(l, i) {
        if (!i || Object.keys(l).length !== Object.keys(i).length) return !1;
        for(const u in l)if (l[u] !== i[u]) return !1;
        return !0;
    }
    function Bh(l) {
        return Array.isArray(l) && l.length === Object.keys(l).length;
    }
    function ro(l) {
        if (!Qh(l)) return !1;
        const i = l.constructor;
        if (i === void 0) return !0;
        const u = i.prototype;
        return !(!Qh(u) || !u.hasOwnProperty("isPrototypeOf") || Object.getPrototypeOf(l) !== Object.prototype);
    }
    function Qh(l) {
        return Object.prototype.toString.call(l) === "[object Object]";
    }
    function Sy(l) {
        return new Promise((i)=>{
            ul.setTimeout(i, l);
        });
    }
    function oo(l, i, u) {
        return typeof u.structuralSharing == "function" ? u.structuralSharing(l, i) : u.structuralSharing !== !1 ? gg(l, i) : i;
    }
    function by(l, i, u = 0) {
        const r = [
            ...l,
            i
        ];
        return u && r.length > u ? r.slice(1) : r;
    }
    function xy(l, i, u = 0) {
        const r = [
            i,
            ...l
        ];
        return u && r.length > u ? r.slice(0, -1) : r;
    }
    var xo = Symbol();
    function mg(l, i) {
        return !l.queryFn && i?.initialPromise ? ()=>i.initialPromise : !l.queryFn || l.queryFn === xo ? ()=>Promise.reject(new Error(`Missing queryFn: '${l.queryHash}'`)) : l.queryFn;
    }
    function _y(l, i) {
        return typeof l == "function" ? l(...i) : !!l;
    }
    var Cy = class extends ei {
        #e;
        #t;
        #n;
        constructor(){
            super(), this.#n = (l)=>{
                if (!sl && window.addEventListener) {
                    const i = ()=>l();
                    return window.addEventListener("visibilitychange", i, !1), ()=>{
                        window.removeEventListener("visibilitychange", i);
                    };
                }
            };
        }
        onSubscribe() {
            this.#t || this.setEventListener(this.#n);
        }
        onUnsubscribe() {
            this.hasListeners() || (this.#t?.(), this.#t = void 0);
        }
        setEventListener(l) {
            this.#n = l, this.#t?.(), this.#t = l((i)=>{
                typeof i == "boolean" ? this.setFocused(i) : this.onFocus();
            });
        }
        setFocused(l) {
            this.#e !== l && (this.#e = l, this.onFocus());
        }
        onFocus() {
            const l = this.isFocused();
            this.listeners.forEach((i)=>{
                i(l);
            });
        }
        isFocused() {
            return typeof this.#e == "boolean" ? this.#e : globalThis.document?.visibilityState !== "hidden";
        }
    }, _o = new Cy;
    function co() {
        let l, i;
        const u = new Promise((c, d)=>{
            l = c, i = d;
        });
        u.status = "pending", u.catch(()=>{});
        function r(c) {
            Object.assign(u, c), delete u.resolve, delete u.reject;
        }
        return u.resolve = (c)=>{
            r({
                status: "fulfilled",
                value: c
            }), l(c);
        }, u.reject = (c)=>{
            r({
                status: "rejected",
                reason: c
            }), i(c);
        }, u;
    }
    var Ry = yy;
    function wy() {
        let l = [], i = 0, u = (v)=>{
            v();
        }, r = (v)=>{
            v();
        }, c = Ry;
        const d = (v)=>{
            i ? l.push(v) : c(()=>{
                u(v);
            });
        }, m = ()=>{
            const v = l;
            l = [], v.length && c(()=>{
                r(()=>{
                    v.forEach((y)=>{
                        u(y);
                    });
                });
            });
        };
        return {
            batch: (v)=>{
                let y;
                i++;
                try {
                    y = v();
                } finally{
                    i--, i || m();
                }
                return y;
            },
            batchCalls: (v)=>(...y)=>{
                    d(()=>{
                        v(...y);
                    });
                },
            schedule: d,
            setNotifyFunction: (v)=>{
                u = v;
            },
            setBatchNotifyFunction: (v)=>{
                r = v;
            },
            setScheduler: (v)=>{
                c = v;
            }
        };
    }
    var Je = wy(), Ey = class extends ei {
        #e = !0;
        #t;
        #n;
        constructor(){
            super(), this.#n = (l)=>{
                if (!sl && window.addEventListener) {
                    const i = ()=>l(!0), u = ()=>l(!1);
                    return window.addEventListener("online", i, !1), window.addEventListener("offline", u, !1), ()=>{
                        window.removeEventListener("online", i), window.removeEventListener("offline", u);
                    };
                }
            };
        }
        onSubscribe() {
            this.#t || this.setEventListener(this.#n);
        }
        onUnsubscribe() {
            this.hasListeners() || (this.#t?.(), this.#t = void 0);
        }
        setEventListener(l) {
            this.#n = l, this.#t?.(), this.#t = l(this.setOnline.bind(this));
        }
        setOnline(l) {
            this.#e !== l && (this.#e = l, this.listeners.forEach((u)=>{
                u(l);
            }));
        }
        isOnline() {
            return this.#e;
        }
    }, Cu = new Ey;
    function My(l) {
        return Math.min(1e3 * 2 ** l, 3e4);
    }
    function yg(l) {
        return (l ?? "online") === "online" ? Cu.isOnline() : !0;
    }
    var fo = class extends Error {
        constructor(l){
            super("CancelledError"), this.revert = l?.revert, this.silent = l?.silent;
        }
    };
    function vg(l) {
        let i = !1, u = 0, r;
        const c = co(), d = ()=>c.status !== "pending", m = (q)=>{
            if (!d()) {
                const B = new fo(q);
                j(B), l.onCancel?.(B);
            }
        }, v = ()=>{
            i = !0;
        }, y = ()=>{
            i = !1;
        }, p = ()=>_o.isFocused() && (l.networkMode === "always" || Cu.isOnline()) && l.canRun(), _ = ()=>yg(l.networkMode) && l.canRun(), M = (q)=>{
            d() || (r?.(), c.resolve(q));
        }, j = (q)=>{
            d() || (r?.(), c.reject(q));
        }, T = ()=>new Promise((q)=>{
                r = (B)=>{
                    (d() || p()) && q(B);
                }, l.onPause?.();
            }).then(()=>{
                r = void 0, d() || l.onContinue?.();
            }), U = ()=>{
            if (d()) return;
            let q;
            const B = u === 0 ? l.initialPromise : void 0;
            try {
                q = B ?? l.fn();
            } catch (G) {
                q = Promise.reject(G);
            }
            Promise.resolve(q).then(M).catch((G)=>{
                if (d()) return;
                const ae = l.retry ?? (sl ? 0 : 3), Z = l.retryDelay ?? My, oe = typeof Z == "function" ? Z(u, G) : Z, he = ae === !0 || typeof ae == "number" && u < ae || typeof ae == "function" && ae(u, G);
                if (i || !he) {
                    j(G);
                    return;
                }
                u++, l.onFail?.(u, G), Sy(oe).then(()=>p() ? void 0 : T()).then(()=>{
                    i ? j(G) : U();
                });
            });
        };
        return {
            promise: c,
            status: ()=>c.status,
            cancel: m,
            continue: ()=>(r?.(), c),
            cancelRetry: v,
            continueRetry: y,
            canStart: _,
            start: ()=>(_() ? U() : T().then(U), c)
        };
    }
    var pg = class {
        #e;
        destroy() {
            this.clearGcTimeout();
        }
        scheduleGc() {
            this.clearGcTimeout(), uo(this.gcTime) && (this.#e = ul.setTimeout(()=>{
                this.optionalRemove();
            }, this.gcTime));
        }
        updateGcTime(l) {
            this.gcTime = Math.max(this.gcTime || 0, l ?? (sl ? 1 / 0 : 300 * 1e3));
        }
        clearGcTimeout() {
            this.#e && (ul.clearTimeout(this.#e), this.#e = void 0);
        }
    }, jy = class extends pg {
        #e;
        #t;
        #n;
        #a;
        #l;
        #s;
        #u;
        constructor(l){
            super(), this.#u = !1, this.#s = l.defaultOptions, this.setOptions(l.options), this.observers = [], this.#a = l.client, this.#n = this.#a.getQueryCache(), this.queryKey = l.queryKey, this.queryHash = l.queryHash, this.#e = Xh(this.options), this.state = l.state ?? this.#e, this.scheduleGc();
        }
        get meta() {
            return this.options.meta;
        }
        get promise() {
            return this.#l?.promise;
        }
        setOptions(l) {
            if (this.options = {
                ...this.#s,
                ...l
            }, this.updateGcTime(this.options.gcTime), this.state && this.state.data === void 0) {
                const i = Xh(this.options);
                i.data !== void 0 && (this.setState(Yh(i.data, i.dataUpdatedAt)), this.#e = i);
            }
        }
        optionalRemove() {
            !this.observers.length && this.state.fetchStatus === "idle" && this.#n.remove(this);
        }
        setData(l, i) {
            const u = oo(this.state.data, l, this.options);
            return this.#i({
                data: u,
                type: "success",
                dataUpdatedAt: i?.updatedAt,
                manual: i?.manual
            }), u;
        }
        setState(l, i) {
            this.#i({
                type: "setState",
                state: l,
                setStateOptions: i
            });
        }
        cancel(l) {
            const i = this.#l?.promise;
            return this.#l?.cancel(l), i ? i.then(ot).catch(ot) : Promise.resolve();
        }
        destroy() {
            super.destroy(), this.cancel({
                silent: !0
            });
        }
        reset() {
            this.destroy(), this.setState(this.#e);
        }
        isActive() {
            return this.observers.some((l)=>Nt(l.options.enabled, this) !== !1);
        }
        isDisabled() {
            return this.getObserversCount() > 0 ? !this.isActive() : this.options.queryFn === xo || this.state.dataUpdateCount + this.state.errorUpdateCount === 0;
        }
        isStatic() {
            return this.getObserversCount() > 0 ? this.observers.some((l)=>qn(l.options.staleTime, this) === "static") : !1;
        }
        isStale() {
            return this.getObserversCount() > 0 ? this.observers.some((l)=>l.getCurrentResult().isStale) : this.state.data === void 0 || this.state.isInvalidated;
        }
        isStaleByTime(l = 0) {
            return this.state.data === void 0 ? !0 : l === "static" ? !1 : this.state.isInvalidated ? !0 : !hg(this.state.dataUpdatedAt, l);
        }
        onFocus() {
            this.observers.find((i)=>i.shouldFetchOnWindowFocus())?.refetch({
                cancelRefetch: !1
            }), this.#l?.continue();
        }
        onOnline() {
            this.observers.find((i)=>i.shouldFetchOnReconnect())?.refetch({
                cancelRefetch: !1
            }), this.#l?.continue();
        }
        addObserver(l) {
            this.observers.includes(l) || (this.observers.push(l), this.clearGcTimeout(), this.#n.notify({
                type: "observerAdded",
                query: this,
                observer: l
            }));
        }
        removeObserver(l) {
            this.observers.includes(l) && (this.observers = this.observers.filter((i)=>i !== l), this.observers.length || (this.#l && (this.#u ? this.#l.cancel({
                revert: !0
            }) : this.#l.cancelRetry()), this.scheduleGc()), this.#n.notify({
                type: "observerRemoved",
                query: this,
                observer: l
            }));
        }
        getObserversCount() {
            return this.observers.length;
        }
        invalidate() {
            this.state.isInvalidated || this.#i({
                type: "invalidate"
            });
        }
        async fetch(l, i) {
            if (this.state.fetchStatus !== "idle" && this.#l?.status() !== "rejected") {
                if (this.state.data !== void 0 && i?.cancelRefetch) this.cancel({
                    silent: !0
                });
                else if (this.#l) return this.#l.continueRetry(), this.#l.promise;
            }
            if (l && this.setOptions(l), !this.options.queryFn) {
                const v = this.observers.find((y)=>y.options.queryFn);
                v && this.setOptions(v.options);
            }
            const u = new AbortController, r = (v)=>{
                Object.defineProperty(v, "signal", {
                    enumerable: !0,
                    get: ()=>(this.#u = !0, u.signal)
                });
            }, c = ()=>{
                const v = mg(this.options, i), p = (()=>{
                    const _ = {
                        client: this.#a,
                        queryKey: this.queryKey,
                        meta: this.meta
                    };
                    return r(_), _;
                })();
                return this.#u = !1, this.options.persister ? this.options.persister(v, p, this) : v(p);
            }, m = (()=>{
                const v = {
                    fetchOptions: i,
                    options: this.options,
                    queryKey: this.queryKey,
                    client: this.#a,
                    state: this.state,
                    fetchFn: c
                };
                return r(v), v;
            })();
            this.options.behavior?.onFetch(m, this), this.#t = this.state, (this.state.fetchStatus === "idle" || this.state.fetchMeta !== m.fetchOptions?.meta) && this.#i({
                type: "fetch",
                meta: m.fetchOptions?.meta
            }), this.#l = vg({
                initialPromise: i?.initialPromise,
                fn: m.fetchFn,
                onCancel: (v)=>{
                    v instanceof fo && v.revert && this.setState({
                        ...this.#t,
                        fetchStatus: "idle"
                    }), u.abort();
                },
                onFail: (v, y)=>{
                    this.#i({
                        type: "failed",
                        failureCount: v,
                        error: y
                    });
                },
                onPause: ()=>{
                    this.#i({
                        type: "pause"
                    });
                },
                onContinue: ()=>{
                    this.#i({
                        type: "continue"
                    });
                },
                retry: m.options.retry,
                retryDelay: m.options.retryDelay,
                networkMode: m.options.networkMode,
                canRun: ()=>!0
            });
            try {
                const v = await this.#l.start();
                if (v === void 0) throw new Error(`${this.queryHash} data is undefined`);
                return this.setData(v), this.#n.config.onSuccess?.(v, this), this.#n.config.onSettled?.(v, this.state.error, this), v;
            } catch (v) {
                if (v instanceof fo) {
                    if (v.silent) return this.#l.promise;
                    if (v.revert) {
                        if (this.state.data === void 0) throw v;
                        return this.state.data;
                    }
                }
                throw this.#i({
                    type: "error",
                    error: v
                }), this.#n.config.onError?.(v, this), this.#n.config.onSettled?.(this.state.data, v, this), v;
            } finally{
                this.scheduleGc();
            }
        }
        #i(l) {
            const i = (u)=>{
                switch(l.type){
                    case "failed":
                        return {
                            ...u,
                            fetchFailureCount: l.failureCount,
                            fetchFailureReason: l.error
                        };
                    case "pause":
                        return {
                            ...u,
                            fetchStatus: "paused"
                        };
                    case "continue":
                        return {
                            ...u,
                            fetchStatus: "fetching"
                        };
                    case "fetch":
                        return {
                            ...u,
                            ...Sg(u.data, this.options),
                            fetchMeta: l.meta ?? null
                        };
                    case "success":
                        const r = {
                            ...u,
                            ...Yh(l.data, l.dataUpdatedAt),
                            dataUpdateCount: u.dataUpdateCount + 1,
                            ...!l.manual && {
                                fetchStatus: "idle",
                                fetchFailureCount: 0,
                                fetchFailureReason: null
                            }
                        };
                        return this.#t = l.manual ? r : void 0, r;
                    case "error":
                        const c = l.error;
                        return {
                            ...u,
                            error: c,
                            errorUpdateCount: u.errorUpdateCount + 1,
                            errorUpdatedAt: Date.now(),
                            fetchFailureCount: u.fetchFailureCount + 1,
                            fetchFailureReason: c,
                            fetchStatus: "idle",
                            status: "error"
                        };
                    case "invalidate":
                        return {
                            ...u,
                            isInvalidated: !0
                        };
                    case "setState":
                        return {
                            ...u,
                            ...l.state
                        };
                }
            };
            this.state = i(this.state), Je.batch(()=>{
                this.observers.forEach((u)=>{
                    u.onQueryUpdate();
                }), this.#n.notify({
                    query: this,
                    type: "updated",
                    action: l
                });
            });
        }
    };
    function Sg(l, i) {
        return {
            fetchFailureCount: 0,
            fetchFailureReason: null,
            fetchStatus: yg(i.networkMode) ? "fetching" : "paused",
            ...l === void 0 && {
                error: null,
                status: "pending"
            }
        };
    }
    function Yh(l, i) {
        return {
            data: l,
            dataUpdatedAt: i ?? Date.now(),
            error: null,
            isInvalidated: !1,
            status: "success"
        };
    }
    function Xh(l) {
        const i = typeof l.initialData == "function" ? l.initialData() : l.initialData, u = i !== void 0, r = u ? typeof l.initialDataUpdatedAt == "function" ? l.initialDataUpdatedAt() : l.initialDataUpdatedAt : 0;
        return {
            data: i,
            dataUpdateCount: 0,
            dataUpdatedAt: u ? r ?? Date.now() : 0,
            error: null,
            errorUpdateCount: 0,
            errorUpdatedAt: 0,
            fetchFailureCount: 0,
            fetchFailureReason: null,
            fetchMeta: null,
            isInvalidated: !1,
            status: u ? "success" : "pending",
            fetchStatus: "idle"
        };
    }
    var Oy = class extends ei {
        constructor(l, i){
            super(), this.options = i, this.#e = l, this.#i = null, this.#u = co(), this.bindMethods(), this.setOptions(i);
        }
        #e;
        #t = void 0;
        #n = void 0;
        #a = void 0;
        #l;
        #s;
        #u;
        #i;
        #m;
        #d;
        #h;
        #o;
        #c;
        #r;
        #g = new Set;
        bindMethods() {
            this.refetch = this.refetch.bind(this);
        }
        onSubscribe() {
            this.listeners.size === 1 && (this.#t.addObserver(this), Fh(this.#t, this.options) ? this.#f() : this.updateResult(), this.#S());
        }
        onUnsubscribe() {
            this.hasListeners() || this.destroy();
        }
        shouldFetchOnReconnect() {
            return ho(this.#t, this.options, this.options.refetchOnReconnect);
        }
        shouldFetchOnWindowFocus() {
            return ho(this.#t, this.options, this.options.refetchOnWindowFocus);
        }
        destroy() {
            this.listeners = new Set, this.#b(), this.#x(), this.#t.removeObserver(this);
        }
        setOptions(l) {
            const i = this.options, u = this.#t;
            if (this.options = this.#e.defaultQueryOptions(l), this.options.enabled !== void 0 && typeof this.options.enabled != "boolean" && typeof this.options.enabled != "function" && typeof Nt(this.options.enabled, this.#t) != "boolean") throw new Error("Expected enabled to be a boolean or a callback that returns a boolean");
            this.#_(), this.#t.setOptions(this.options), i._defaulted && !so(this.options, i) && this.#e.getQueryCache().notify({
                type: "observerOptionsUpdated",
                query: this.#t,
                observer: this
            });
            const r = this.hasListeners();
            r && Zh(this.#t, u, this.options, i) && this.#f(), this.updateResult(), r && (this.#t !== u || Nt(this.options.enabled, this.#t) !== Nt(i.enabled, this.#t) || qn(this.options.staleTime, this.#t) !== qn(i.staleTime, this.#t)) && this.#y();
            const c = this.#v();
            r && (this.#t !== u || Nt(this.options.enabled, this.#t) !== Nt(i.enabled, this.#t) || c !== this.#r) && this.#p(c);
        }
        getOptimisticResult(l) {
            const i = this.#e.getQueryCache().build(this.#e, l), u = this.createResult(i, l);
            return Ny(this, u) && (this.#a = u, this.#s = this.options, this.#l = this.#t.state), u;
        }
        getCurrentResult() {
            return this.#a;
        }
        trackResult(l, i) {
            return new Proxy(l, {
                get: (u, r)=>(this.trackProp(r), i?.(r), r === "promise" && (this.trackProp("data"), !this.options.experimental_prefetchInRender && this.#u.status === "pending" && this.#u.reject(new Error("experimental_prefetchInRender feature flag is not enabled"))), Reflect.get(u, r))
            });
        }
        trackProp(l) {
            this.#g.add(l);
        }
        getCurrentQuery() {
            return this.#t;
        }
        refetch({ ...l } = {}) {
            return this.fetch({
                ...l
            });
        }
        fetchOptimistic(l) {
            const i = this.#e.defaultQueryOptions(l), u = this.#e.getQueryCache().build(this.#e, i);
            return u.fetch().then(()=>this.createResult(u, i));
        }
        fetch(l) {
            return this.#f({
                ...l,
                cancelRefetch: l.cancelRefetch ?? !0
            }).then(()=>(this.updateResult(), this.#a));
        }
        #f(l) {
            this.#_();
            let i = this.#t.fetch(this.options, l);
            return l?.throwOnError || (i = i.catch(ot)), i;
        }
        #y() {
            this.#b();
            const l = qn(this.options.staleTime, this.#t);
            if (sl || this.#a.isStale || !uo(l)) return;
            const u = hg(this.#a.dataUpdatedAt, l) + 1;
            this.#o = ul.setTimeout(()=>{
                this.#a.isStale || this.updateResult();
            }, u);
        }
        #v() {
            return (typeof this.options.refetchInterval == "function" ? this.options.refetchInterval(this.#t) : this.options.refetchInterval) ?? !1;
        }
        #p(l) {
            this.#x(), this.#r = l, !(sl || Nt(this.options.enabled, this.#t) === !1 || !uo(this.#r) || this.#r === 0) && (this.#c = ul.setInterval(()=>{
                (this.options.refetchIntervalInBackground || _o.isFocused()) && this.#f();
            }, this.#r));
        }
        #S() {
            this.#y(), this.#p(this.#v());
        }
        #b() {
            this.#o && (ul.clearTimeout(this.#o), this.#o = void 0);
        }
        #x() {
            this.#c && (ul.clearInterval(this.#c), this.#c = void 0);
        }
        createResult(l, i) {
            const u = this.#t, r = this.options, c = this.#a, d = this.#l, m = this.#s, y = l !== u ? l.state : this.#n, { state: p } = l;
            let _ = {
                ...p
            }, M = !1, j;
            if (i._optimisticResults) {
                const I = this.hasListeners(), Me = !I && Fh(l, i), Pe = I && Zh(l, u, i, r);
                (Me || Pe) && (_ = {
                    ..._,
                    ...Sg(p.data, l.options)
                }), i._optimisticResults === "isRestoring" && (_.fetchStatus = "idle");
            }
            let { error: T, errorUpdatedAt: U, status: q } = _;
            j = _.data;
            let B = !1;
            if (i.placeholderData !== void 0 && j === void 0 && q === "pending") {
                let I;
                c?.isPlaceholderData && i.placeholderData === m?.placeholderData ? (I = c.data, B = !0) : I = typeof i.placeholderData == "function" ? i.placeholderData(this.#h?.state.data, this.#h) : i.placeholderData, I !== void 0 && (q = "success", j = oo(c?.data, I, i), M = !0);
            }
            if (i.select && j !== void 0 && !B) if (c && j === d?.data && i.select === this.#m) j = this.#d;
            else try {
                this.#m = i.select, j = i.select(j), j = oo(c?.data, j, i), this.#d = j, this.#i = null;
            } catch (I) {
                this.#i = I;
            }
            this.#i && (T = this.#i, j = this.#d, U = Date.now(), q = "error");
            const G = _.fetchStatus === "fetching", ae = q === "pending", Z = q === "error", oe = ae && G, he = j !== void 0, $ = {
                status: q,
                fetchStatus: _.fetchStatus,
                isPending: ae,
                isSuccess: q === "success",
                isError: Z,
                isInitialLoading: oe,
                isLoading: oe,
                data: j,
                dataUpdatedAt: _.dataUpdatedAt,
                error: T,
                errorUpdatedAt: U,
                failureCount: _.fetchFailureCount,
                failureReason: _.fetchFailureReason,
                errorUpdateCount: _.errorUpdateCount,
                isFetched: _.dataUpdateCount > 0 || _.errorUpdateCount > 0,
                isFetchedAfterMount: _.dataUpdateCount > y.dataUpdateCount || _.errorUpdateCount > y.errorUpdateCount,
                isFetching: G,
                isRefetching: G && !ae,
                isLoadingError: Z && !he,
                isPaused: _.fetchStatus === "paused",
                isPlaceholderData: M,
                isRefetchError: Z && he,
                isStale: Co(l, i),
                refetch: this.refetch,
                promise: this.#u,
                isEnabled: Nt(i.enabled, l) !== !1
            };
            if (this.options.experimental_prefetchInRender) {
                const I = (Be)=>{
                    $.status === "error" ? Be.reject($.error) : $.data !== void 0 && Be.resolve($.data);
                }, Me = ()=>{
                    const Be = this.#u = $.promise = co();
                    I(Be);
                }, Pe = this.#u;
                switch(Pe.status){
                    case "pending":
                        l.queryHash === u.queryHash && I(Pe);
                        break;
                    case "fulfilled":
                        ($.status === "error" || $.data !== Pe.value) && Me();
                        break;
                    case "rejected":
                        ($.status !== "error" || $.error !== Pe.reason) && Me();
                        break;
                }
            }
            return $;
        }
        updateResult() {
            const l = this.#a, i = this.createResult(this.#t, this.options);
            if (this.#l = this.#t.state, this.#s = this.options, this.#l.data !== void 0 && (this.#h = this.#t), so(i, l)) return;
            this.#a = i;
            const u = ()=>{
                if (!l) return !0;
                const { notifyOnChangeProps: r } = this.options, c = typeof r == "function" ? r() : r;
                if (c === "all" || !c && !this.#g.size) return !0;
                const d = new Set(c ?? this.#g);
                return this.options.throwOnError && d.add("error"), Object.keys(this.#a).some((m)=>{
                    const v = m;
                    return this.#a[v] !== l[v] && d.has(v);
                });
            };
            this.#C({
                listeners: u()
            });
        }
        #_() {
            const l = this.#e.getQueryCache().build(this.#e, this.options);
            if (l === this.#t) return;
            const i = this.#t;
            this.#t = l, this.#n = l.state, this.hasListeners() && (i?.removeObserver(this), l.addObserver(this));
        }
        onQueryUpdate() {
            this.updateResult(), this.hasListeners() && this.#S();
        }
        #C(l) {
            Je.batch(()=>{
                l.listeners && this.listeners.forEach((i)=>{
                    i(this.#a);
                }), this.#e.getQueryCache().notify({
                    query: this.#t,
                    type: "observerResultsUpdated"
                });
            });
        }
    };
    function Ty(l, i) {
        return Nt(i.enabled, l) !== !1 && l.state.data === void 0 && !(l.state.status === "error" && i.retryOnMount === !1);
    }
    function Fh(l, i) {
        return Ty(l, i) || l.state.data !== void 0 && ho(l, i, i.refetchOnMount);
    }
    function ho(l, i, u) {
        if (Nt(i.enabled, l) !== !1 && qn(i.staleTime, l) !== "static") {
            const r = typeof u == "function" ? u(l) : u;
            return r === "always" || r !== !1 && Co(l, i);
        }
        return !1;
    }
    function Zh(l, i, u, r) {
        return (l !== i || Nt(r.enabled, l) === !1) && (!u.suspense || l.state.status !== "error") && Co(l, u);
    }
    function Co(l, i) {
        return Nt(i.enabled, l) !== !1 && l.isStaleByTime(qn(i.staleTime, l));
    }
    function Ny(l, i) {
        return !so(l.getCurrentResult(), i);
    }
    function Kh(l) {
        return {
            onFetch: (i, u)=>{
                const r = i.options, c = i.fetchOptions?.meta?.fetchMore?.direction, d = i.state.data?.pages || [], m = i.state.data?.pageParams || [];
                let v = {
                    pages: [],
                    pageParams: []
                }, y = 0;
                const p = async ()=>{
                    let _ = !1;
                    const M = (U)=>{
                        Object.defineProperty(U, "signal", {
                            enumerable: !0,
                            get: ()=>(i.signal.aborted ? _ = !0 : i.signal.addEventListener("abort", ()=>{
                                    _ = !0;
                                }), i.signal)
                        });
                    }, j = mg(i.options, i.fetchOptions), T = async (U, q, B)=>{
                        if (_) return Promise.reject();
                        if (q == null && U.pages.length) return Promise.resolve(U);
                        const ae = (()=>{
                            const F = {
                                client: i.client,
                                queryKey: i.queryKey,
                                pageParam: q,
                                direction: B ? "backward" : "forward",
                                meta: i.options.meta
                            };
                            return M(F), F;
                        })(), Z = await j(ae), { maxPages: oe } = i.options, he = B ? xy : by;
                        return {
                            pages: he(U.pages, Z, oe),
                            pageParams: he(U.pageParams, q, oe)
                        };
                    };
                    if (c && d.length) {
                        const U = c === "backward", q = U ? Ay : $h, B = {
                            pages: d,
                            pageParams: m
                        }, G = q(r, B);
                        v = await T(B, G, U);
                    } else {
                        const U = l ?? d.length;
                        do {
                            const q = y === 0 ? m[0] ?? r.initialPageParam : $h(r, v);
                            if (y > 0 && q == null) break;
                            v = await T(v, q), y++;
                        }while (y < U);
                    }
                    return v;
                };
                i.options.persister ? i.fetchFn = ()=>i.options.persister?.(p, {
                        client: i.client,
                        queryKey: i.queryKey,
                        meta: i.options.meta,
                        signal: i.signal
                    }, u) : i.fetchFn = p;
            }
        };
    }
    function $h(l, { pages: i, pageParams: u }) {
        const r = i.length - 1;
        return i.length > 0 ? l.getNextPageParam(i[r], i, u[r], u) : void 0;
    }
    function Ay(l, { pages: i, pageParams: u }) {
        return i.length > 0 ? l.getPreviousPageParam?.(i[0], i, u[0], u) : void 0;
    }
    var zy = class extends pg {
        #e;
        #t;
        #n;
        #a;
        constructor(l){
            super(), this.#e = l.client, this.mutationId = l.mutationId, this.#n = l.mutationCache, this.#t = [], this.state = l.state || Dy(), this.setOptions(l.options), this.scheduleGc();
        }
        setOptions(l) {
            this.options = l, this.updateGcTime(this.options.gcTime);
        }
        get meta() {
            return this.options.meta;
        }
        addObserver(l) {
            this.#t.includes(l) || (this.#t.push(l), this.clearGcTimeout(), this.#n.notify({
                type: "observerAdded",
                mutation: this,
                observer: l
            }));
        }
        removeObserver(l) {
            this.#t = this.#t.filter((i)=>i !== l), this.scheduleGc(), this.#n.notify({
                type: "observerRemoved",
                mutation: this,
                observer: l
            });
        }
        optionalRemove() {
            this.#t.length || (this.state.status === "pending" ? this.scheduleGc() : this.#n.remove(this));
        }
        continue() {
            return this.#a?.continue() ?? this.execute(this.state.variables);
        }
        async execute(l) {
            const i = ()=>{
                this.#l({
                    type: "continue"
                });
            }, u = {
                client: this.#e,
                meta: this.options.meta,
                mutationKey: this.options.mutationKey
            };
            this.#a = vg({
                fn: ()=>this.options.mutationFn ? this.options.mutationFn(l, u) : Promise.reject(new Error("No mutationFn found")),
                onFail: (d, m)=>{
                    this.#l({
                        type: "failed",
                        failureCount: d,
                        error: m
                    });
                },
                onPause: ()=>{
                    this.#l({
                        type: "pause"
                    });
                },
                onContinue: i,
                retry: this.options.retry ?? 0,
                retryDelay: this.options.retryDelay,
                networkMode: this.options.networkMode,
                canRun: ()=>this.#n.canRun(this)
            });
            const r = this.state.status === "pending", c = !this.#a.canStart();
            try {
                if (r) i();
                else {
                    this.#l({
                        type: "pending",
                        variables: l,
                        isPaused: c
                    }), await this.#n.config.onMutate?.(l, this, u);
                    const m = await this.options.onMutate?.(l, u);
                    m !== this.state.context && this.#l({
                        type: "pending",
                        context: m,
                        variables: l,
                        isPaused: c
                    });
                }
                const d = await this.#a.start();
                return await this.#n.config.onSuccess?.(d, l, this.state.context, this, u), await this.options.onSuccess?.(d, l, this.state.context, u), await this.#n.config.onSettled?.(d, null, this.state.variables, this.state.context, this, u), await this.options.onSettled?.(d, null, l, this.state.context, u), this.#l({
                    type: "success",
                    data: d
                }), d;
            } catch (d) {
                try {
                    throw await this.#n.config.onError?.(d, l, this.state.context, this, u), await this.options.onError?.(d, l, this.state.context, u), await this.#n.config.onSettled?.(void 0, d, this.state.variables, this.state.context, this, u), await this.options.onSettled?.(void 0, d, l, this.state.context, u), d;
                } finally{
                    this.#l({
                        type: "error",
                        error: d
                    });
                }
            } finally{
                this.#n.runNext(this);
            }
        }
        #l(l) {
            const i = (u)=>{
                switch(l.type){
                    case "failed":
                        return {
                            ...u,
                            failureCount: l.failureCount,
                            failureReason: l.error
                        };
                    case "pause":
                        return {
                            ...u,
                            isPaused: !0
                        };
                    case "continue":
                        return {
                            ...u,
                            isPaused: !1
                        };
                    case "pending":
                        return {
                            ...u,
                            context: l.context,
                            data: void 0,
                            failureCount: 0,
                            failureReason: null,
                            error: null,
                            isPaused: l.isPaused,
                            status: "pending",
                            variables: l.variables,
                            submittedAt: Date.now()
                        };
                    case "success":
                        return {
                            ...u,
                            data: l.data,
                            failureCount: 0,
                            failureReason: null,
                            error: null,
                            status: "success",
                            isPaused: !1
                        };
                    case "error":
                        return {
                            ...u,
                            data: void 0,
                            error: l.error,
                            failureCount: u.failureCount + 1,
                            failureReason: l.error,
                            isPaused: !1,
                            status: "error"
                        };
                }
            };
            this.state = i(this.state), Je.batch(()=>{
                this.#t.forEach((u)=>{
                    u.onMutationUpdate(l);
                }), this.#n.notify({
                    mutation: this,
                    type: "updated",
                    action: l
                });
            });
        }
    };
    function Dy() {
        return {
            context: void 0,
            data: void 0,
            error: null,
            failureCount: 0,
            failureReason: null,
            isPaused: !1,
            status: "idle",
            variables: void 0,
            submittedAt: 0
        };
    }
    var Hy = class extends ei {
        constructor(l = {}){
            super(), this.config = l, this.#e = new Set, this.#t = new Map, this.#n = 0;
        }
        #e;
        #t;
        #n;
        build(l, i, u) {
            const r = new zy({
                client: l,
                mutationCache: this,
                mutationId: ++this.#n,
                options: l.defaultMutationOptions(i),
                state: u
            });
            return this.add(r), r;
        }
        add(l) {
            this.#e.add(l);
            const i = Su(l);
            if (typeof i == "string") {
                const u = this.#t.get(i);
                u ? u.push(l) : this.#t.set(i, [
                    l
                ]);
            }
            this.notify({
                type: "added",
                mutation: l
            });
        }
        remove(l) {
            if (this.#e.delete(l)) {
                const i = Su(l);
                if (typeof i == "string") {
                    const u = this.#t.get(i);
                    if (u) if (u.length > 1) {
                        const r = u.indexOf(l);
                        r !== -1 && u.splice(r, 1);
                    } else u[0] === l && this.#t.delete(i);
                }
            }
            this.notify({
                type: "removed",
                mutation: l
            });
        }
        canRun(l) {
            const i = Su(l);
            if (typeof i == "string") {
                const r = this.#t.get(i)?.find((c)=>c.state.status === "pending");
                return !r || r === l;
            } else return !0;
        }
        runNext(l) {
            const i = Su(l);
            return typeof i == "string" ? this.#t.get(i)?.find((r)=>r !== l && r.state.isPaused)?.continue() ?? Promise.resolve() : Promise.resolve();
        }
        clear() {
            Je.batch(()=>{
                this.#e.forEach((l)=>{
                    this.notify({
                        type: "removed",
                        mutation: l
                    });
                }), this.#e.clear(), this.#t.clear();
            });
        }
        getAll() {
            return Array.from(this.#e);
        }
        find(l) {
            const i = {
                exact: !0,
                ...l
            };
            return this.getAll().find((u)=>Gh(i, u));
        }
        findAll(l = {}) {
            return this.getAll().filter((i)=>Gh(l, i));
        }
        notify(l) {
            Je.batch(()=>{
                this.listeners.forEach((i)=>{
                    i(l);
                });
            });
        }
        resumePausedMutations() {
            const l = this.getAll().filter((i)=>i.state.isPaused);
            return Je.batch(()=>Promise.all(l.map((i)=>i.continue().catch(ot))));
        }
    };
    function Su(l) {
        return l.options.scope?.id;
    }
    var qy = class extends ei {
        constructor(l = {}){
            super(), this.config = l, this.#e = new Map;
        }
        #e;
        build(l, i, u) {
            const r = i.queryKey, c = i.queryHash ?? bo(r, i);
            let d = this.get(c);
            return d || (d = new jy({
                client: l,
                queryKey: r,
                queryHash: c,
                options: l.defaultQueryOptions(i),
                state: u,
                defaultOptions: l.getQueryDefaults(r)
            }), this.add(d)), d;
        }
        add(l) {
            this.#e.has(l.queryHash) || (this.#e.set(l.queryHash, l), this.notify({
                type: "added",
                query: l
            }));
        }
        remove(l) {
            const i = this.#e.get(l.queryHash);
            i && (l.destroy(), i === l && this.#e.delete(l.queryHash), this.notify({
                type: "removed",
                query: l
            }));
        }
        clear() {
            Je.batch(()=>{
                this.getAll().forEach((l)=>{
                    this.remove(l);
                });
            });
        }
        get(l) {
            return this.#e.get(l);
        }
        getAll() {
            return [
                ...this.#e.values()
            ];
        }
        find(l) {
            const i = {
                exact: !0,
                ...l
            };
            return this.getAll().find((u)=>Lh(i, u));
        }
        findAll(l = {}) {
            const i = this.getAll();
            return Object.keys(l).length > 0 ? i.filter((u)=>Lh(l, u)) : i;
        }
        notify(l) {
            Je.batch(()=>{
                this.listeners.forEach((i)=>{
                    i(l);
                });
            });
        }
        onFocus() {
            Je.batch(()=>{
                this.getAll().forEach((l)=>{
                    l.onFocus();
                });
            });
        }
        onOnline() {
            Je.batch(()=>{
                this.getAll().forEach((l)=>{
                    l.onOnline();
                });
            });
        }
    }, Uy = class {
        #e;
        #t;
        #n;
        #a;
        #l;
        #s;
        #u;
        #i;
        constructor(l = {}){
            this.#e = l.queryCache || new qy, this.#t = l.mutationCache || new Hy, this.#n = l.defaultOptions || {}, this.#a = new Map, this.#l = new Map, this.#s = 0;
        }
        mount() {
            this.#s++, this.#s === 1 && (this.#u = _o.subscribe(async (l)=>{
                l && (await this.resumePausedMutations(), this.#e.onFocus());
            }), this.#i = Cu.subscribe(async (l)=>{
                l && (await this.resumePausedMutations(), this.#e.onOnline());
            }));
        }
        unmount() {
            this.#s--, this.#s === 0 && (this.#u?.(), this.#u = void 0, this.#i?.(), this.#i = void 0);
        }
        isFetching(l) {
            return this.#e.findAll({
                ...l,
                fetchStatus: "fetching"
            }).length;
        }
        isMutating(l) {
            return this.#t.findAll({
                ...l,
                status: "pending"
            }).length;
        }
        getQueryData(l) {
            const i = this.defaultQueryOptions({
                queryKey: l
            });
            return this.#e.get(i.queryHash)?.state.data;
        }
        ensureQueryData(l) {
            const i = this.defaultQueryOptions(l), u = this.#e.build(this, i), r = u.state.data;
            return r === void 0 ? this.fetchQuery(l) : (l.revalidateIfStale && u.isStaleByTime(qn(i.staleTime, u)) && this.prefetchQuery(i), Promise.resolve(r));
        }
        getQueriesData(l) {
            return this.#e.findAll(l).map(({ queryKey: i, state: u })=>{
                const r = u.data;
                return [
                    i,
                    r
                ];
            });
        }
        setQueryData(l, i, u) {
            const r = this.defaultQueryOptions({
                queryKey: l
            }), d = this.#e.get(r.queryHash)?.state.data, m = vy(i, d);
            if (m !== void 0) return this.#e.build(this, r).setData(m, {
                ...u,
                manual: !0
            });
        }
        setQueriesData(l, i, u) {
            return Je.batch(()=>this.#e.findAll(l).map(({ queryKey: r })=>[
                        r,
                        this.setQueryData(r, i, u)
                    ]));
        }
        getQueryState(l) {
            const i = this.defaultQueryOptions({
                queryKey: l
            });
            return this.#e.get(i.queryHash)?.state;
        }
        removeQueries(l) {
            const i = this.#e;
            Je.batch(()=>{
                i.findAll(l).forEach((u)=>{
                    i.remove(u);
                });
            });
        }
        resetQueries(l, i) {
            const u = this.#e;
            return Je.batch(()=>(u.findAll(l).forEach((r)=>{
                    r.reset();
                }), this.refetchQueries({
                    type: "active",
                    ...l
                }, i)));
        }
        cancelQueries(l, i = {}) {
            const u = {
                revert: !0,
                ...i
            }, r = Je.batch(()=>this.#e.findAll(l).map((c)=>c.cancel(u)));
            return Promise.all(r).then(ot).catch(ot);
        }
        invalidateQueries(l, i = {}) {
            return Je.batch(()=>(this.#e.findAll(l).forEach((u)=>{
                    u.invalidate();
                }), l?.refetchType === "none" ? Promise.resolve() : this.refetchQueries({
                    ...l,
                    type: l?.refetchType ?? l?.type ?? "active"
                }, i)));
        }
        refetchQueries(l, i = {}) {
            const u = {
                ...i,
                cancelRefetch: i.cancelRefetch ?? !0
            }, r = Je.batch(()=>this.#e.findAll(l).filter((c)=>!c.isDisabled() && !c.isStatic()).map((c)=>{
                    let d = c.fetch(void 0, u);
                    return u.throwOnError || (d = d.catch(ot)), c.state.fetchStatus === "paused" ? Promise.resolve() : d;
                }));
            return Promise.all(r).then(ot);
        }
        fetchQuery(l) {
            const i = this.defaultQueryOptions(l);
            i.retry === void 0 && (i.retry = !1);
            const u = this.#e.build(this, i);
            return u.isStaleByTime(qn(i.staleTime, u)) ? u.fetch(i) : Promise.resolve(u.state.data);
        }
        prefetchQuery(l) {
            return this.fetchQuery(l).then(ot).catch(ot);
        }
        fetchInfiniteQuery(l) {
            return l.behavior = Kh(l.pages), this.fetchQuery(l);
        }
        prefetchInfiniteQuery(l) {
            return this.fetchInfiniteQuery(l).then(ot).catch(ot);
        }
        ensureInfiniteQueryData(l) {
            return l.behavior = Kh(l.pages), this.ensureQueryData(l);
        }
        resumePausedMutations() {
            return Cu.isOnline() ? this.#t.resumePausedMutations() : Promise.resolve();
        }
        getQueryCache() {
            return this.#e;
        }
        getMutationCache() {
            return this.#t;
        }
        getDefaultOptions() {
            return this.#n;
        }
        setDefaultOptions(l) {
            this.#n = l;
        }
        setQueryDefaults(l, i) {
            this.#a.set(Wa(l), {
                queryKey: l,
                defaultOptions: i
            });
        }
        getQueryDefaults(l) {
            const i = [
                ...this.#a.values()
            ], u = {};
            return i.forEach((r)=>{
                Pa(l, r.queryKey) && Object.assign(u, r.defaultOptions);
            }), u;
        }
        setMutationDefaults(l, i) {
            this.#l.set(Wa(l), {
                mutationKey: l,
                defaultOptions: i
            });
        }
        getMutationDefaults(l) {
            const i = [
                ...this.#l.values()
            ], u = {};
            return i.forEach((r)=>{
                Pa(l, r.mutationKey) && Object.assign(u, r.defaultOptions);
            }), u;
        }
        defaultQueryOptions(l) {
            if (l._defaulted) return l;
            const i = {
                ...this.#n.queries,
                ...this.getQueryDefaults(l.queryKey),
                ...l,
                _defaulted: !0
            };
            return i.queryHash || (i.queryHash = bo(i.queryKey, i)), i.refetchOnReconnect === void 0 && (i.refetchOnReconnect = i.networkMode !== "always"), i.throwOnError === void 0 && (i.throwOnError = !!i.suspense), !i.networkMode && i.persister && (i.networkMode = "offlineFirst"), i.queryFn === xo && (i.enabled = !1), i;
        }
        defaultMutationOptions(l) {
            return l?._defaulted ? l : {
                ...this.#n.mutations,
                ...l?.mutationKey && this.getMutationDefaults(l.mutationKey),
                ...l,
                _defaulted: !0
            };
        }
        clear() {
            this.#e.clear(), this.#t.clear();
        }
    }, bg = ie.createContext(void 0), Vy = (l)=>{
        const i = ie.useContext(bg);
        if (!i) throw new Error("No QueryClient set, use QueryClientProvider to set one");
        return i;
    }, Ly = ({ client: l, children: i })=>(ie.useEffect(()=>(l.mount(), ()=>{
                l.unmount();
            }), [
            l
        ]), h.jsx(bg.Provider, {
            value: l,
            children: i
        })), xg = ie.createContext(!1), Gy = ()=>ie.useContext(xg);
    xg.Provider;
    function By() {
        let l = !1;
        return {
            clearReset: ()=>{
                l = !1;
            },
            reset: ()=>{
                l = !0;
            },
            isReset: ()=>l
        };
    }
    var Qy = ie.createContext(By()), Yy = ()=>ie.useContext(Qy), Xy = (l, i)=>{
        (l.suspense || l.throwOnError || l.experimental_prefetchInRender) && (i.isReset() || (l.retryOnMount = !1));
    }, Fy = (l)=>{
        ie.useEffect(()=>{
            l.clearReset();
        }, [
            l
        ]);
    }, Zy = ({ result: l, errorResetBoundary: i, throwOnError: u, query: r, suspense: c })=>l.isError && !i.isReset() && !l.isFetching && r && (c && l.data === void 0 || _y(u, [
            l.error,
            r
        ])), Ky = (l)=>{
        if (l.suspense) {
            const u = (c)=>c === "static" ? c : Math.max(c ?? 1e3, 1e3), r = l.staleTime;
            l.staleTime = typeof r == "function" ? (...c)=>u(r(...c)) : u(r), typeof l.gcTime == "number" && (l.gcTime = Math.max(l.gcTime, 1e3));
        }
    }, $y = (l, i)=>l.isLoading && l.isFetching && !i, Jy = (l, i)=>l?.suspense && i.isPending, Jh = (l, i, u)=>i.fetchOptimistic(l).catch(()=>{
            u.clearReset();
        });
    function ky(l, i, u) {
        const r = Gy(), c = Yy(), d = Vy(), m = d.defaultQueryOptions(l);
        d.getDefaultOptions().queries?._experimental_beforeQuery?.(m), m._optimisticResults = r ? "isRestoring" : "optimistic", Ky(m), Xy(m, c), Fy(c);
        const v = !d.getQueryCache().get(m.queryHash), [y] = ie.useState(()=>new i(d, m)), p = y.getOptimisticResult(m), _ = !r && l.subscribed !== !1;
        if (ie.useSyncExternalStore(ie.useCallback((M)=>{
            const j = _ ? y.subscribe(Je.batchCalls(M)) : ot;
            return y.updateResult(), j;
        }, [
            y,
            _
        ]), ()=>y.getCurrentResult(), ()=>y.getCurrentResult()), ie.useEffect(()=>{
            y.setOptions(m);
        }, [
            m,
            y
        ]), Jy(m, p)) throw Jh(m, y, c);
        if (Zy({
            result: p,
            errorResetBoundary: c,
            throwOnError: m.throwOnError,
            query: d.getQueryCache().get(m.queryHash),
            suspense: m.suspense
        })) throw p.error;
        return d.getDefaultOptions().queries?._experimental_afterQuery?.(m, p), m.experimental_prefetchInRender && !sl && $y(p, r) && (v ? Jh(m, y, c) : d.getQueryCache().get(m.queryHash)?.promise)?.catch(ot).finally(()=>{
            y.updateResult();
        }), m.notifyOnChangeProps ? p : y.trackResult(p);
    }
    function Kl(l, i) {
        return ky(l, Oy);
    }
    class Wy extends ie.Component {
        constructor(i){
            super(i), this.state = {
                hasError: !1,
                error: null
            };
        }
        static getDerivedStateFromError(i) {
            return {
                hasError: !0,
                error: i
            };
        }
        componentDidCatch(i, u) {
            console.error("Error caught by boundary:", i, u);
        }
        render() {
            return this.state.hasError ? this.props.fallback ? this.props.fallback : h.jsx("div", {
                className: "min-h-screen bg-gray-50 flex items-center justify-center p-4",
                children: h.jsxs("div", {
                    className: "bg-white shadow-sm border border-red-200 rounded-lg p-6 max-w-lg w-full",
                    children: [
                        h.jsxs("div", {
                            className: "flex items-center gap-3 mb-4",
                            children: [
                                h.jsx("svg", {
                                    className: "w-8 h-8 text-red-500",
                                    fill: "none",
                                    viewBox: "0 0 24 24",
                                    stroke: "currentColor",
                                    children: h.jsx("path", {
                                        strokeLinecap: "round",
                                        strokeLinejoin: "round",
                                        strokeWidth: 2,
                                        d: "M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
                                    })
                                }),
                                h.jsx("h2", {
                                    className: "text-lg font-semibold text-gray-900",
                                    children: "Something went wrong"
                                })
                            ]
                        }),
                        h.jsx("p", {
                            className: "text-sm text-gray-600 mb-4",
                            children: "An unexpected error occurred while rendering the application."
                        }),
                        this.state.error && h.jsx("pre", {
                            className: "bg-gray-100 p-3 rounded text-xs text-gray-700 overflow-auto max-h-32 mb-4",
                            children: this.state.error.message
                        }),
                        h.jsx("button", {
                            onClick: ()=>window.location.reload(),
                            className: "px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 text-sm font-medium",
                            children: "Reload Page"
                        })
                    ]
                })
            }) : this.props.children;
        }
    }
    function Py({ metadata: l, filters: i, onFilterChange: u }) {
        const r = (y, p)=>{
            u({
                ...i,
                [y]: p
            });
        }, c = ie.useMemo(()=>i.labels?.split(",").filter(Boolean) || [], [
            i.labels
        ]), d = ie.useMemo(()=>new Set(c), [
            c
        ]), m = (y, p)=>{
            const _ = p ? [
                ...c,
                y
            ] : c.filter((M)=>M !== y);
            r("labels", _.length > 0 ? _.join(",") : void 0);
        }, v = ()=>{
            u({
                limit: 1e3
            });
        };
        return h.jsxs("div", {
            className: "bg-white shadow-sm border border-gray-200 rounded-lg p-6 space-y-6",
            children: [
                h.jsxs("div", {
                    className: "flex items-center justify-between",
                    children: [
                        h.jsx("h2", {
                            className: "text-lg font-semibold text-gray-900",
                            children: "Filters"
                        }),
                        h.jsx("button", {
                            onClick: v,
                            className: "text-sm text-blue-600 hover:text-blue-800",
                            children: "Revert to Default View"
                        })
                    ]
                }),
                h.jsxs("div", {
                    className: "space-y-6",
                    children: [
                        h.jsxs("div", {
                            className: "border-b border-gray-200 pb-4",
                            children: [
                                h.jsx("h3", {
                                    className: "text-sm font-semibold text-gray-900 mb-3",
                                    children: "Well Known Issues"
                                }),
                                h.jsx("div", {
                                    children: h.jsxs("label", {
                                        className: "flex items-center space-x-2",
                                        children: [
                                            h.jsx("input", {
                                                type: "checkbox",
                                                checked: i.has_resolution_or_discussion_url || !1,
                                                onChange: (y)=>r("has_resolution_or_discussion_url", y.target.checked || void 0),
                                                className: "rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                                            }),
                                            h.jsx("span", {
                                                className: "text-sm text-gray-700",
                                                children: "Has a resolution or discussion URL"
                                            })
                                        ]
                                    })
                                })
                            ]
                        }),
                        h.jsxs("div", {
                            className: "border-b border-gray-200 pb-4",
                            children: [
                                h.jsx("h3", {
                                    className: "text-sm font-semibold text-gray-900 mb-3",
                                    children: "Time Range"
                                }),
                                h.jsxs("div", {
                                    className: "space-y-4",
                                    children: [
                                        h.jsxs("div", {
                                            children: [
                                                h.jsx("label", {
                                                    className: "block text-sm font-medium text-gray-700 mb-1",
                                                    children: "Since Time"
                                                }),
                                                h.jsx("input", {
                                                    type: "text",
                                                    value: i.since_time || "",
                                                    onChange: (y)=>r("since_time", y.target.value),
                                                    placeholder: "2025-10-27 or '2 days ago'",
                                                    className: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                                })
                                            ]
                                        }),
                                        h.jsxs("div", {
                                            children: [
                                                h.jsx("label", {
                                                    className: "block text-sm font-medium text-gray-700 mb-1",
                                                    children: "To Time"
                                                }),
                                                h.jsx("input", {
                                                    type: "text",
                                                    value: i.to_time || "",
                                                    onChange: (y)=>r("to_time", y.target.value),
                                                    placeholder: "2025-10-27 or 'now'",
                                                    className: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                                })
                                            ]
                                        })
                                    ]
                                })
                            ]
                        }),
                        h.jsxs("div", {
                            className: "border-b border-gray-200 pb-4",
                            children: [
                                h.jsx("h3", {
                                    className: "text-sm font-semibold text-gray-900 mb-3",
                                    children: "Zoom In"
                                }),
                                h.jsxs("div", {
                                    className: "space-y-4",
                                    children: [
                                        h.jsxs("div", {
                                            children: [
                                                h.jsx("label", {
                                                    className: "block text-sm font-medium text-gray-700 mb-1",
                                                    children: "Node"
                                                }),
                                                h.jsxs("select", {
                                                    value: i.node || "",
                                                    onChange: (y)=>r("node", y.target.value || void 0),
                                                    className: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                                                    children: [
                                                        h.jsx("option", {
                                                            value: "",
                                                            children: "All"
                                                        }),
                                                        l?.nodes.map((y)=>h.jsx("option", {
                                                                value: y,
                                                                children: y
                                                            }, y))
                                                    ]
                                                })
                                            ]
                                        }),
                                        h.jsxs("div", {
                                            children: [
                                                h.jsx("label", {
                                                    className: "block text-sm font-medium text-gray-700 mb-1",
                                                    children: "Subsystem"
                                                }),
                                                h.jsxs("select", {
                                                    value: i.subsystem || "",
                                                    onChange: (y)=>r("subsystem", y.target.value || void 0),
                                                    className: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                                                    children: [
                                                        h.jsx("option", {
                                                            value: "",
                                                            children: "All"
                                                        }),
                                                        l?.subsystems.map((y)=>h.jsx("option", {
                                                                value: y,
                                                                children: y
                                                            }, y))
                                                    ]
                                                })
                                            ]
                                        }),
                                        h.jsxs("div", {
                                            children: [
                                                h.jsx("label", {
                                                    className: "block text-sm font-medium text-gray-700 mb-1",
                                                    children: "Erlang PID"
                                                }),
                                                h.jsxs("div", {
                                                    className: "flex gap-2",
                                                    children: [
                                                        h.jsx("input", {
                                                            type: "text",
                                                            value: i.erlang_pid || "",
                                                            onChange: (y)=>r("erlang_pid", y.target.value || void 0),
                                                            placeholder: "<0.208.0>",
                                                            className: "flex-1 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                                        }),
                                                        h.jsx("button", {
                                                            onClick: ()=>r("erlang_pid", void 0),
                                                            className: "px-3 py-2 text-sm font-medium text-gray-700 bg-gray-100 border border-gray-300 rounded-md hover:bg-gray-200 focus:outline-none focus:ring-2 focus:ring-blue-500",
                                                            children: "Clear"
                                                        })
                                                    ]
                                                })
                                            ]
                                        })
                                    ]
                                })
                            ]
                        }),
                        h.jsxs("div", {
                            className: "border-b border-gray-200 pb-4",
                            children: [
                                h.jsx("h3", {
                                    className: "text-sm font-semibold text-gray-900 mb-3",
                                    children: "Zoom In Further"
                                }),
                                h.jsxs("div", {
                                    children: [
                                        h.jsx("label", {
                                            className: "block text-sm font-medium text-gray-700 mb-2",
                                            children: "Labels"
                                        }),
                                        h.jsx("div", {
                                            className: "space-y-2 max-h-96 overflow-y-auto",
                                            children: l?.labels.map((y)=>h.jsxs("label", {
                                                    className: "flex items-center space-x-2",
                                                    children: [
                                                        h.jsx("input", {
                                                            type: "checkbox",
                                                            checked: d.has(y),
                                                            onChange: (p)=>m(y, p.target.checked),
                                                            className: "rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                                                        }),
                                                        h.jsx("span", {
                                                            className: "text-sm text-gray-700",
                                                            children: y
                                                        })
                                                    ]
                                                }, y))
                                        })
                                    ]
                                }),
                                c.length > 1 && h.jsx("div", {
                                    className: "mt-4",
                                    children: h.jsxs("label", {
                                        className: "flex items-center space-x-2",
                                        children: [
                                            h.jsx("input", {
                                                type: "checkbox",
                                                checked: i.matching_all_labels || !1,
                                                onChange: (y)=>r("matching_all_labels", y.target.checked),
                                                className: "rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                                            }),
                                            h.jsx("span", {
                                                className: "text-sm text-gray-700",
                                                children: "Show entries that match all selected labels (an AND query)"
                                            })
                                        ]
                                    })
                                }),
                                h.jsxs("div", {
                                    className: "mt-4",
                                    children: [
                                        h.jsx("label", {
                                            className: "block text-sm font-medium text-gray-700 mb-1",
                                            children: "Severity"
                                        }),
                                        h.jsxs("select", {
                                            value: i.severity || "",
                                            onChange: (y)=>r("severity", y.target.value || void 0),
                                            className: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                                            children: [
                                                h.jsx("option", {
                                                    value: "",
                                                    children: "All"
                                                }),
                                                l?.severities.map((y)=>h.jsx("option", {
                                                        value: y,
                                                        children: y
                                                    }, y))
                                            ]
                                        })
                                    ]
                                })
                            ]
                        }),
                        h.jsxs("div", {
                            children: [
                                h.jsx("label", {
                                    className: "block text-sm font-medium text-gray-700 mb-1",
                                    children: "Log entry rows to load (maximum)"
                                }),
                                h.jsxs("select", {
                                    value: i.limit || 1e3,
                                    onChange: (y)=>r("limit", parseInt(y.target.value)),
                                    className: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                                    children: [
                                        h.jsx("option", {
                                            value: 500,
                                            children: "500"
                                        }),
                                        h.jsx("option", {
                                            value: 1e3,
                                            children: "1000"
                                        }),
                                        h.jsx("option", {
                                            value: 3e3,
                                            children: "3000"
                                        }),
                                        h.jsx("option", {
                                            value: 5e3,
                                            children: "5000"
                                        }),
                                        h.jsx("option", {
                                            value: 1e4,
                                            children: "10000"
                                        }),
                                        h.jsx("option", {
                                            value: 2e4,
                                            children: "20000"
                                        }),
                                        h.jsx("option", {
                                            value: 5e4,
                                            children: "50000"
                                        })
                                    ]
                                })
                            ]
                        })
                    ]
                })
            ]
        });
    }
    function Hn(l, i) {
        return typeof l == "function" ? l(i) : l;
    }
    function bt(l, i) {
        return (u)=>{
            i.setState((r)=>({
                    ...r,
                    [l]: Hn(u, r[l])
                }));
        };
    }
    function wu(l) {
        return l instanceof Function;
    }
    function Iy(l) {
        return Array.isArray(l) && l.every((i)=>typeof i == "number");
    }
    function ev(l, i) {
        const u = [], r = (c)=>{
            c.forEach((d)=>{
                u.push(d);
                const m = i(d);
                m != null && m.length && r(m);
            });
        };
        return r(l), u;
    }
    function W(l, i, u) {
        let r = [], c;
        return (d)=>{
            let m;
            u.key && u.debug && (m = Date.now());
            const v = l(d);
            if (!(v.length !== r.length || v.some((_, M)=>r[M] !== _))) return c;
            r = v;
            let p;
            if (u.key && u.debug && (p = Date.now()), c = i(...v), u == null || u.onChange == null || u.onChange(c), u.key && u.debug && u != null && u.debug()) {
                const _ = Math.round((Date.now() - m) * 100) / 100, M = Math.round((Date.now() - p) * 100) / 100, j = M / 16, T = (U, q)=>{
                    for(U = String(U); U.length < q;)U = " " + U;
                    return U;
                };
                console.info(`%c⏱ ${T(M, 5)} /${T(_, 5)} ms`, `
            font-size: .6rem;
            font-weight: bold;
            color: hsl(${Math.max(0, Math.min(120 - 120 * j, 120))}deg 100% 31%);`, u?.key);
            }
            return c;
        };
    }
    function P(l, i, u, r) {
        return {
            debug: ()=>{
                var c;
                return (c = l?.debugAll) != null ? c : l[i];
            },
            key: !1,
            onChange: r
        };
    }
    function tv(l, i, u, r) {
        const c = ()=>{
            var m;
            return (m = d.getValue()) != null ? m : l.options.renderFallbackValue;
        }, d = {
            id: `${i.id}_${u.id}`,
            row: i,
            column: u,
            getValue: ()=>i.getValue(r),
            renderValue: c,
            getContext: W(()=>[
                    l,
                    u,
                    i,
                    d
                ], (m, v, y, p)=>({
                    table: m,
                    column: v,
                    row: y,
                    cell: p,
                    getValue: p.getValue,
                    renderValue: p.renderValue
                }), P(l.options, "debugCells"))
        };
        return l._features.forEach((m)=>{
            m.createCell == null || m.createCell(d, u, i, l);
        }, {}), d;
    }
    function nv(l, i, u, r) {
        var c, d;
        const v = {
            ...l._getDefaultColumnDef(),
            ...i
        }, y = v.accessorKey;
        let p = (c = (d = v.id) != null ? d : y ? typeof String.prototype.replaceAll == "function" ? y.replaceAll(".", "_") : y.replace(/\./g, "_") : void 0) != null ? c : typeof v.header == "string" ? v.header : void 0, _;
        if (v.accessorFn ? _ = v.accessorFn : y && (y.includes(".") ? _ = (j)=>{
            let T = j;
            for (const q of y.split(".")){
                var U;
                T = (U = T) == null ? void 0 : U[q];
            }
            return T;
        } : _ = (j)=>j[v.accessorKey]), !p) throw new Error;
        let M = {
            id: `${String(p)}`,
            accessorFn: _,
            parent: r,
            depth: u,
            columnDef: v,
            columns: [],
            getFlatColumns: W(()=>[
                    !0
                ], ()=>{
                var j;
                return [
                    M,
                    ...(j = M.columns) == null ? void 0 : j.flatMap((T)=>T.getFlatColumns())
                ];
            }, P(l.options, "debugColumns")),
            getLeafColumns: W(()=>[
                    l._getOrderColumnsFn()
                ], (j)=>{
                var T;
                if ((T = M.columns) != null && T.length) {
                    let U = M.columns.flatMap((q)=>q.getLeafColumns());
                    return j(U);
                }
                return [
                    M
                ];
            }, P(l.options, "debugColumns"))
        };
        for (const j of l._features)j.createColumn == null || j.createColumn(M, l);
        return M;
    }
    const We = "debugHeaders";
    function kh(l, i, u) {
        var r;
        let d = {
            id: (r = u.id) != null ? r : i.id,
            column: i,
            index: u.index,
            isPlaceholder: !!u.isPlaceholder,
            placeholderId: u.placeholderId,
            depth: u.depth,
            subHeaders: [],
            colSpan: 0,
            rowSpan: 0,
            headerGroup: null,
            getLeafHeaders: ()=>{
                const m = [], v = (y)=>{
                    y.subHeaders && y.subHeaders.length && y.subHeaders.map(v), m.push(y);
                };
                return v(d), m;
            },
            getContext: ()=>({
                    table: l,
                    header: d,
                    column: i
                })
        };
        return l._features.forEach((m)=>{
            m.createHeader == null || m.createHeader(d, l);
        }), d;
    }
    const lv = {
        createTable: (l)=>{
            l.getHeaderGroups = W(()=>[
                    l.getAllColumns(),
                    l.getVisibleLeafColumns(),
                    l.getState().columnPinning.left,
                    l.getState().columnPinning.right
                ], (i, u, r, c)=>{
                var d, m;
                const v = (d = r?.map((M)=>u.find((j)=>j.id === M)).filter(Boolean)) != null ? d : [], y = (m = c?.map((M)=>u.find((j)=>j.id === M)).filter(Boolean)) != null ? m : [], p = u.filter((M)=>!(r != null && r.includes(M.id)) && !(c != null && c.includes(M.id)));
                return bu(i, [
                    ...v,
                    ...p,
                    ...y
                ], l);
            }, P(l.options, We)), l.getCenterHeaderGroups = W(()=>[
                    l.getAllColumns(),
                    l.getVisibleLeafColumns(),
                    l.getState().columnPinning.left,
                    l.getState().columnPinning.right
                ], (i, u, r, c)=>(u = u.filter((d)=>!(r != null && r.includes(d.id)) && !(c != null && c.includes(d.id))), bu(i, u, l, "center")), P(l.options, We)), l.getLeftHeaderGroups = W(()=>[
                    l.getAllColumns(),
                    l.getVisibleLeafColumns(),
                    l.getState().columnPinning.left
                ], (i, u, r)=>{
                var c;
                const d = (c = r?.map((m)=>u.find((v)=>v.id === m)).filter(Boolean)) != null ? c : [];
                return bu(i, d, l, "left");
            }, P(l.options, We)), l.getRightHeaderGroups = W(()=>[
                    l.getAllColumns(),
                    l.getVisibleLeafColumns(),
                    l.getState().columnPinning.right
                ], (i, u, r)=>{
                var c;
                const d = (c = r?.map((m)=>u.find((v)=>v.id === m)).filter(Boolean)) != null ? c : [];
                return bu(i, d, l, "right");
            }, P(l.options, We)), l.getFooterGroups = W(()=>[
                    l.getHeaderGroups()
                ], (i)=>[
                    ...i
                ].reverse(), P(l.options, We)), l.getLeftFooterGroups = W(()=>[
                    l.getLeftHeaderGroups()
                ], (i)=>[
                    ...i
                ].reverse(), P(l.options, We)), l.getCenterFooterGroups = W(()=>[
                    l.getCenterHeaderGroups()
                ], (i)=>[
                    ...i
                ].reverse(), P(l.options, We)), l.getRightFooterGroups = W(()=>[
                    l.getRightHeaderGroups()
                ], (i)=>[
                    ...i
                ].reverse(), P(l.options, We)), l.getFlatHeaders = W(()=>[
                    l.getHeaderGroups()
                ], (i)=>i.map((u)=>u.headers).flat(), P(l.options, We)), l.getLeftFlatHeaders = W(()=>[
                    l.getLeftHeaderGroups()
                ], (i)=>i.map((u)=>u.headers).flat(), P(l.options, We)), l.getCenterFlatHeaders = W(()=>[
                    l.getCenterHeaderGroups()
                ], (i)=>i.map((u)=>u.headers).flat(), P(l.options, We)), l.getRightFlatHeaders = W(()=>[
                    l.getRightHeaderGroups()
                ], (i)=>i.map((u)=>u.headers).flat(), P(l.options, We)), l.getCenterLeafHeaders = W(()=>[
                    l.getCenterFlatHeaders()
                ], (i)=>i.filter((u)=>{
                    var r;
                    return !((r = u.subHeaders) != null && r.length);
                }), P(l.options, We)), l.getLeftLeafHeaders = W(()=>[
                    l.getLeftFlatHeaders()
                ], (i)=>i.filter((u)=>{
                    var r;
                    return !((r = u.subHeaders) != null && r.length);
                }), P(l.options, We)), l.getRightLeafHeaders = W(()=>[
                    l.getRightFlatHeaders()
                ], (i)=>i.filter((u)=>{
                    var r;
                    return !((r = u.subHeaders) != null && r.length);
                }), P(l.options, We)), l.getLeafHeaders = W(()=>[
                    l.getLeftHeaderGroups(),
                    l.getCenterHeaderGroups(),
                    l.getRightHeaderGroups()
                ], (i, u, r)=>{
                var c, d, m, v, y, p;
                return [
                    ...(c = (d = i[0]) == null ? void 0 : d.headers) != null ? c : [],
                    ...(m = (v = u[0]) == null ? void 0 : v.headers) != null ? m : [],
                    ...(y = (p = r[0]) == null ? void 0 : p.headers) != null ? y : []
                ].map((_)=>_.getLeafHeaders()).flat();
            }, P(l.options, We));
        }
    };
    function bu(l, i, u, r) {
        var c, d;
        let m = 0;
        const v = function(j, T) {
            T === void 0 && (T = 1), m = Math.max(m, T), j.filter((U)=>U.getIsVisible()).forEach((U)=>{
                var q;
                (q = U.columns) != null && q.length && v(U.columns, T + 1);
            }, 0);
        };
        v(l);
        let y = [];
        const p = (j, T)=>{
            const U = {
                depth: T,
                id: [
                    r,
                    `${T}`
                ].filter(Boolean).join("_"),
                headers: []
            }, q = [];
            j.forEach((B)=>{
                const G = [
                    ...q
                ].reverse()[0], ae = B.column.depth === U.depth;
                let Z, oe = !1;
                if (ae && B.column.parent ? Z = B.column.parent : (Z = B.column, oe = !0), G && G?.column === Z) G.subHeaders.push(B);
                else {
                    const he = kh(u, Z, {
                        id: [
                            r,
                            T,
                            Z.id,
                            B?.id
                        ].filter(Boolean).join("_"),
                        isPlaceholder: oe,
                        placeholderId: oe ? `${q.filter((F)=>F.column === Z).length}` : void 0,
                        depth: T,
                        index: q.length
                    });
                    he.subHeaders.push(B), q.push(he);
                }
                U.headers.push(B), B.headerGroup = U;
            }), y.push(U), T > 0 && p(q, T - 1);
        }, _ = i.map((j, T)=>kh(u, j, {
                depth: m,
                index: T
            }));
        p(_, m - 1), y.reverse();
        const M = (j)=>j.filter((U)=>U.column.getIsVisible()).map((U)=>{
                let q = 0, B = 0, G = [
                    0
                ];
                U.subHeaders && U.subHeaders.length ? (G = [], M(U.subHeaders).forEach((Z)=>{
                    let { colSpan: oe, rowSpan: he } = Z;
                    q += oe, G.push(he);
                })) : q = 1;
                const ae = Math.min(...G);
                return B = B + ae, U.colSpan = q, U.rowSpan = B, {
                    colSpan: q,
                    rowSpan: B
                };
            });
        return M((c = (d = y[0]) == null ? void 0 : d.headers) != null ? c : []), y;
    }
    const av = (l, i, u, r, c, d, m)=>{
        let v = {
            id: i,
            index: r,
            original: u,
            depth: c,
            parentId: m,
            _valuesCache: {},
            _uniqueValuesCache: {},
            getValue: (y)=>{
                if (v._valuesCache.hasOwnProperty(y)) return v._valuesCache[y];
                const p = l.getColumn(y);
                if (p != null && p.accessorFn) return v._valuesCache[y] = p.accessorFn(v.original, r), v._valuesCache[y];
            },
            getUniqueValues: (y)=>{
                if (v._uniqueValuesCache.hasOwnProperty(y)) return v._uniqueValuesCache[y];
                const p = l.getColumn(y);
                if (p != null && p.accessorFn) return p.columnDef.getUniqueValues ? (v._uniqueValuesCache[y] = p.columnDef.getUniqueValues(v.original, r), v._uniqueValuesCache[y]) : (v._uniqueValuesCache[y] = [
                    v.getValue(y)
                ], v._uniqueValuesCache[y]);
            },
            renderValue: (y)=>{
                var p;
                return (p = v.getValue(y)) != null ? p : l.options.renderFallbackValue;
            },
            subRows: [],
            getLeafRows: ()=>ev(v.subRows, (y)=>y.subRows),
            getParentRow: ()=>v.parentId ? l.getRow(v.parentId, !0) : void 0,
            getParentRows: ()=>{
                let y = [], p = v;
                for(;;){
                    const _ = p.getParentRow();
                    if (!_) break;
                    y.push(_), p = _;
                }
                return y.reverse();
            },
            getAllCells: W(()=>[
                    l.getAllLeafColumns()
                ], (y)=>y.map((p)=>tv(l, v, p, p.id)), P(l.options, "debugRows")),
            _getAllCellsByColumnId: W(()=>[
                    v.getAllCells()
                ], (y)=>y.reduce((p, _)=>(p[_.column.id] = _, p), {}), P(l.options, "debugRows"))
        };
        for(let y = 0; y < l._features.length; y++){
            const p = l._features[y];
            p == null || p.createRow == null || p.createRow(v, l);
        }
        return v;
    }, iv = {
        createColumn: (l, i)=>{
            l._getFacetedRowModel = i.options.getFacetedRowModel && i.options.getFacetedRowModel(i, l.id), l.getFacetedRowModel = ()=>l._getFacetedRowModel ? l._getFacetedRowModel() : i.getPreFilteredRowModel(), l._getFacetedUniqueValues = i.options.getFacetedUniqueValues && i.options.getFacetedUniqueValues(i, l.id), l.getFacetedUniqueValues = ()=>l._getFacetedUniqueValues ? l._getFacetedUniqueValues() : new Map, l._getFacetedMinMaxValues = i.options.getFacetedMinMaxValues && i.options.getFacetedMinMaxValues(i, l.id), l.getFacetedMinMaxValues = ()=>{
                if (l._getFacetedMinMaxValues) return l._getFacetedMinMaxValues();
            };
        }
    }, _g = (l, i, u)=>{
        var r, c;
        const d = u == null || (r = u.toString()) == null ? void 0 : r.toLowerCase();
        return !!(!((c = l.getValue(i)) == null || (c = c.toString()) == null || (c = c.toLowerCase()) == null) && c.includes(d));
    };
    _g.autoRemove = (l)=>qt(l);
    const Cg = (l, i, u)=>{
        var r;
        return !!(!((r = l.getValue(i)) == null || (r = r.toString()) == null) && r.includes(u));
    };
    Cg.autoRemove = (l)=>qt(l);
    const Rg = (l, i, u)=>{
        var r;
        return ((r = l.getValue(i)) == null || (r = r.toString()) == null ? void 0 : r.toLowerCase()) === u?.toLowerCase();
    };
    Rg.autoRemove = (l)=>qt(l);
    const wg = (l, i, u)=>{
        var r;
        return (r = l.getValue(i)) == null ? void 0 : r.includes(u);
    };
    wg.autoRemove = (l)=>qt(l);
    const Eg = (l, i, u)=>!u.some((r)=>{
            var c;
            return !((c = l.getValue(i)) != null && c.includes(r));
        });
    Eg.autoRemove = (l)=>qt(l) || !(l != null && l.length);
    const Mg = (l, i, u)=>u.some((r)=>{
            var c;
            return (c = l.getValue(i)) == null ? void 0 : c.includes(r);
        });
    Mg.autoRemove = (l)=>qt(l) || !(l != null && l.length);
    const jg = (l, i, u)=>l.getValue(i) === u;
    jg.autoRemove = (l)=>qt(l);
    const Og = (l, i, u)=>l.getValue(i) == u;
    Og.autoRemove = (l)=>qt(l);
    const Ro = (l, i, u)=>{
        let [r, c] = u;
        const d = l.getValue(i);
        return d >= r && d <= c;
    };
    Ro.resolveFilterValue = (l)=>{
        let [i, u] = l, r = typeof i != "number" ? parseFloat(i) : i, c = typeof u != "number" ? parseFloat(u) : u, d = i === null || Number.isNaN(r) ? -1 / 0 : r, m = u === null || Number.isNaN(c) ? 1 / 0 : c;
        if (d > m) {
            const v = d;
            d = m, m = v;
        }
        return [
            d,
            m
        ];
    };
    Ro.autoRemove = (l)=>qt(l) || qt(l[0]) && qt(l[1]);
    const rn = {
        includesString: _g,
        includesStringSensitive: Cg,
        equalsString: Rg,
        arrIncludes: wg,
        arrIncludesAll: Eg,
        arrIncludesSome: Mg,
        equals: jg,
        weakEquals: Og,
        inNumberRange: Ro
    };
    function qt(l) {
        return l == null || l === "";
    }
    const uv = {
        getDefaultColumnDef: ()=>({
                filterFn: "auto"
            }),
        getInitialState: (l)=>({
                columnFilters: [],
                ...l
            }),
        getDefaultOptions: (l)=>({
                onColumnFiltersChange: bt("columnFilters", l),
                filterFromLeafRows: !1,
                maxLeafRowFilterDepth: 100
            }),
        createColumn: (l, i)=>{
            l.getAutoFilterFn = ()=>{
                const u = i.getCoreRowModel().flatRows[0], r = u?.getValue(l.id);
                return typeof r == "string" ? rn.includesString : typeof r == "number" ? rn.inNumberRange : typeof r == "boolean" || r !== null && typeof r == "object" ? rn.equals : Array.isArray(r) ? rn.arrIncludes : rn.weakEquals;
            }, l.getFilterFn = ()=>{
                var u, r;
                return wu(l.columnDef.filterFn) ? l.columnDef.filterFn : l.columnDef.filterFn === "auto" ? l.getAutoFilterFn() : (u = (r = i.options.filterFns) == null ? void 0 : r[l.columnDef.filterFn]) != null ? u : rn[l.columnDef.filterFn];
            }, l.getCanFilter = ()=>{
                var u, r, c;
                return ((u = l.columnDef.enableColumnFilter) != null ? u : !0) && ((r = i.options.enableColumnFilters) != null ? r : !0) && ((c = i.options.enableFilters) != null ? c : !0) && !!l.accessorFn;
            }, l.getIsFiltered = ()=>l.getFilterIndex() > -1, l.getFilterValue = ()=>{
                var u;
                return (u = i.getState().columnFilters) == null || (u = u.find((r)=>r.id === l.id)) == null ? void 0 : u.value;
            }, l.getFilterIndex = ()=>{
                var u, r;
                return (u = (r = i.getState().columnFilters) == null ? void 0 : r.findIndex((c)=>c.id === l.id)) != null ? u : -1;
            }, l.setFilterValue = (u)=>{
                i.setColumnFilters((r)=>{
                    const c = l.getFilterFn(), d = r?.find((_)=>_.id === l.id), m = Hn(u, d ? d.value : void 0);
                    if (Wh(c, m, l)) {
                        var v;
                        return (v = r?.filter((_)=>_.id !== l.id)) != null ? v : [];
                    }
                    const y = {
                        id: l.id,
                        value: m
                    };
                    if (d) {
                        var p;
                        return (p = r?.map((_)=>_.id === l.id ? y : _)) != null ? p : [];
                    }
                    return r != null && r.length ? [
                        ...r,
                        y
                    ] : [
                        y
                    ];
                });
            };
        },
        createRow: (l, i)=>{
            l.columnFilters = {}, l.columnFiltersMeta = {};
        },
        createTable: (l)=>{
            l.setColumnFilters = (i)=>{
                const u = l.getAllLeafColumns(), r = (c)=>{
                    var d;
                    return (d = Hn(i, c)) == null ? void 0 : d.filter((m)=>{
                        const v = u.find((y)=>y.id === m.id);
                        if (v) {
                            const y = v.getFilterFn();
                            if (Wh(y, m.value, v)) return !1;
                        }
                        return !0;
                    });
                };
                l.options.onColumnFiltersChange == null || l.options.onColumnFiltersChange(r);
            }, l.resetColumnFilters = (i)=>{
                var u, r;
                l.setColumnFilters(i ? [] : (u = (r = l.initialState) == null ? void 0 : r.columnFilters) != null ? u : []);
            }, l.getPreFilteredRowModel = ()=>l.getCoreRowModel(), l.getFilteredRowModel = ()=>(!l._getFilteredRowModel && l.options.getFilteredRowModel && (l._getFilteredRowModel = l.options.getFilteredRowModel(l)), l.options.manualFiltering || !l._getFilteredRowModel ? l.getPreFilteredRowModel() : l._getFilteredRowModel());
        }
    };
    function Wh(l, i, u) {
        return (l && l.autoRemove ? l.autoRemove(i, u) : !1) || typeof i > "u" || typeof i == "string" && !i;
    }
    const sv = (l, i, u)=>u.reduce((r, c)=>{
            const d = c.getValue(l);
            return r + (typeof d == "number" ? d : 0);
        }, 0), rv = (l, i, u)=>{
        let r;
        return u.forEach((c)=>{
            const d = c.getValue(l);
            d != null && (r > d || r === void 0 && d >= d) && (r = d);
        }), r;
    }, ov = (l, i, u)=>{
        let r;
        return u.forEach((c)=>{
            const d = c.getValue(l);
            d != null && (r < d || r === void 0 && d >= d) && (r = d);
        }), r;
    }, cv = (l, i, u)=>{
        let r, c;
        return u.forEach((d)=>{
            const m = d.getValue(l);
            m != null && (r === void 0 ? m >= m && (r = c = m) : (r > m && (r = m), c < m && (c = m)));
        }), [
            r,
            c
        ];
    }, fv = (l, i)=>{
        let u = 0, r = 0;
        if (i.forEach((c)=>{
            let d = c.getValue(l);
            d != null && (d = +d) >= d && (++u, r += d);
        }), u) return r / u;
    }, dv = (l, i)=>{
        if (!i.length) return;
        const u = i.map((d)=>d.getValue(l));
        if (!Iy(u)) return;
        if (u.length === 1) return u[0];
        const r = Math.floor(u.length / 2), c = u.sort((d, m)=>d - m);
        return u.length % 2 !== 0 ? c[r] : (c[r - 1] + c[r]) / 2;
    }, hv = (l, i)=>Array.from(new Set(i.map((u)=>u.getValue(l))).values()), gv = (l, i)=>new Set(i.map((u)=>u.getValue(l))).size, mv = (l, i)=>i.length, Wr = {
        sum: sv,
        min: rv,
        max: ov,
        extent: cv,
        mean: fv,
        median: dv,
        unique: hv,
        uniqueCount: gv,
        count: mv
    }, yv = {
        getDefaultColumnDef: ()=>({
                aggregatedCell: (l)=>{
                    var i, u;
                    return (i = (u = l.getValue()) == null || u.toString == null ? void 0 : u.toString()) != null ? i : null;
                },
                aggregationFn: "auto"
            }),
        getInitialState: (l)=>({
                grouping: [],
                ...l
            }),
        getDefaultOptions: (l)=>({
                onGroupingChange: bt("grouping", l),
                groupedColumnMode: "reorder"
            }),
        createColumn: (l, i)=>{
            l.toggleGrouping = ()=>{
                i.setGrouping((u)=>u != null && u.includes(l.id) ? u.filter((r)=>r !== l.id) : [
                        ...u ?? [],
                        l.id
                    ]);
            }, l.getCanGroup = ()=>{
                var u, r;
                return ((u = l.columnDef.enableGrouping) != null ? u : !0) && ((r = i.options.enableGrouping) != null ? r : !0) && (!!l.accessorFn || !!l.columnDef.getGroupingValue);
            }, l.getIsGrouped = ()=>{
                var u;
                return (u = i.getState().grouping) == null ? void 0 : u.includes(l.id);
            }, l.getGroupedIndex = ()=>{
                var u;
                return (u = i.getState().grouping) == null ? void 0 : u.indexOf(l.id);
            }, l.getToggleGroupingHandler = ()=>{
                const u = l.getCanGroup();
                return ()=>{
                    u && l.toggleGrouping();
                };
            }, l.getAutoAggregationFn = ()=>{
                const u = i.getCoreRowModel().flatRows[0], r = u?.getValue(l.id);
                if (typeof r == "number") return Wr.sum;
                if (Object.prototype.toString.call(r) === "[object Date]") return Wr.extent;
            }, l.getAggregationFn = ()=>{
                var u, r;
                if (!l) throw new Error;
                return wu(l.columnDef.aggregationFn) ? l.columnDef.aggregationFn : l.columnDef.aggregationFn === "auto" ? l.getAutoAggregationFn() : (u = (r = i.options.aggregationFns) == null ? void 0 : r[l.columnDef.aggregationFn]) != null ? u : Wr[l.columnDef.aggregationFn];
            };
        },
        createTable: (l)=>{
            l.setGrouping = (i)=>l.options.onGroupingChange == null ? void 0 : l.options.onGroupingChange(i), l.resetGrouping = (i)=>{
                var u, r;
                l.setGrouping(i ? [] : (u = (r = l.initialState) == null ? void 0 : r.grouping) != null ? u : []);
            }, l.getPreGroupedRowModel = ()=>l.getFilteredRowModel(), l.getGroupedRowModel = ()=>(!l._getGroupedRowModel && l.options.getGroupedRowModel && (l._getGroupedRowModel = l.options.getGroupedRowModel(l)), l.options.manualGrouping || !l._getGroupedRowModel ? l.getPreGroupedRowModel() : l._getGroupedRowModel());
        },
        createRow: (l, i)=>{
            l.getIsGrouped = ()=>!!l.groupingColumnId, l.getGroupingValue = (u)=>{
                if (l._groupingValuesCache.hasOwnProperty(u)) return l._groupingValuesCache[u];
                const r = i.getColumn(u);
                return r != null && r.columnDef.getGroupingValue ? (l._groupingValuesCache[u] = r.columnDef.getGroupingValue(l.original), l._groupingValuesCache[u]) : l.getValue(u);
            }, l._groupingValuesCache = {};
        },
        createCell: (l, i, u, r)=>{
            l.getIsGrouped = ()=>i.getIsGrouped() && i.id === u.groupingColumnId, l.getIsPlaceholder = ()=>!l.getIsGrouped() && i.getIsGrouped(), l.getIsAggregated = ()=>{
                var c;
                return !l.getIsGrouped() && !l.getIsPlaceholder() && !!((c = u.subRows) != null && c.length);
            };
        }
    };
    function vv(l, i, u) {
        if (!(i != null && i.length) || !u) return l;
        const r = l.filter((d)=>!i.includes(d.id));
        return u === "remove" ? r : [
            ...i.map((d)=>l.find((m)=>m.id === d)).filter(Boolean),
            ...r
        ];
    }
    const pv = {
        getInitialState: (l)=>({
                columnOrder: [],
                ...l
            }),
        getDefaultOptions: (l)=>({
                onColumnOrderChange: bt("columnOrder", l)
            }),
        createColumn: (l, i)=>{
            l.getIndex = W((u)=>[
                    ka(i, u)
                ], (u)=>u.findIndex((r)=>r.id === l.id), P(i.options, "debugColumns")), l.getIsFirstColumn = (u)=>{
                var r;
                return ((r = ka(i, u)[0]) == null ? void 0 : r.id) === l.id;
            }, l.getIsLastColumn = (u)=>{
                var r;
                const c = ka(i, u);
                return ((r = c[c.length - 1]) == null ? void 0 : r.id) === l.id;
            };
        },
        createTable: (l)=>{
            l.setColumnOrder = (i)=>l.options.onColumnOrderChange == null ? void 0 : l.options.onColumnOrderChange(i), l.resetColumnOrder = (i)=>{
                var u;
                l.setColumnOrder(i ? [] : (u = l.initialState.columnOrder) != null ? u : []);
            }, l._getOrderColumnsFn = W(()=>[
                    l.getState().columnOrder,
                    l.getState().grouping,
                    l.options.groupedColumnMode
                ], (i, u, r)=>(c)=>{
                    let d = [];
                    if (!(i != null && i.length)) d = c;
                    else {
                        const m = [
                            ...i
                        ], v = [
                            ...c
                        ];
                        for(; v.length && m.length;){
                            const y = m.shift(), p = v.findIndex((_)=>_.id === y);
                            p > -1 && d.push(v.splice(p, 1)[0]);
                        }
                        d = [
                            ...d,
                            ...v
                        ];
                    }
                    return vv(d, u, r);
                }, P(l.options, "debugTable"));
        }
    }, Pr = ()=>({
            left: [],
            right: []
        }), Sv = {
        getInitialState: (l)=>({
                columnPinning: Pr(),
                ...l
            }),
        getDefaultOptions: (l)=>({
                onColumnPinningChange: bt("columnPinning", l)
            }),
        createColumn: (l, i)=>{
            l.pin = (u)=>{
                const r = l.getLeafColumns().map((c)=>c.id).filter(Boolean);
                i.setColumnPinning((c)=>{
                    var d, m;
                    if (u === "right") {
                        var v, y;
                        return {
                            left: ((v = c?.left) != null ? v : []).filter((M)=>!(r != null && r.includes(M))),
                            right: [
                                ...((y = c?.right) != null ? y : []).filter((M)=>!(r != null && r.includes(M))),
                                ...r
                            ]
                        };
                    }
                    if (u === "left") {
                        var p, _;
                        return {
                            left: [
                                ...((p = c?.left) != null ? p : []).filter((M)=>!(r != null && r.includes(M))),
                                ...r
                            ],
                            right: ((_ = c?.right) != null ? _ : []).filter((M)=>!(r != null && r.includes(M)))
                        };
                    }
                    return {
                        left: ((d = c?.left) != null ? d : []).filter((M)=>!(r != null && r.includes(M))),
                        right: ((m = c?.right) != null ? m : []).filter((M)=>!(r != null && r.includes(M)))
                    };
                });
            }, l.getCanPin = ()=>l.getLeafColumns().some((r)=>{
                    var c, d, m;
                    return ((c = r.columnDef.enablePinning) != null ? c : !0) && ((d = (m = i.options.enableColumnPinning) != null ? m : i.options.enablePinning) != null ? d : !0);
                }), l.getIsPinned = ()=>{
                const u = l.getLeafColumns().map((v)=>v.id), { left: r, right: c } = i.getState().columnPinning, d = u.some((v)=>r?.includes(v)), m = u.some((v)=>c?.includes(v));
                return d ? "left" : m ? "right" : !1;
            }, l.getPinnedIndex = ()=>{
                var u, r;
                const c = l.getIsPinned();
                return c ? (u = (r = i.getState().columnPinning) == null || (r = r[c]) == null ? void 0 : r.indexOf(l.id)) != null ? u : -1 : 0;
            };
        },
        createRow: (l, i)=>{
            l.getCenterVisibleCells = W(()=>[
                    l._getAllVisibleCells(),
                    i.getState().columnPinning.left,
                    i.getState().columnPinning.right
                ], (u, r, c)=>{
                const d = [
                    ...r ?? [],
                    ...c ?? []
                ];
                return u.filter((m)=>!d.includes(m.column.id));
            }, P(i.options, "debugRows")), l.getLeftVisibleCells = W(()=>[
                    l._getAllVisibleCells(),
                    i.getState().columnPinning.left
                ], (u, r)=>(r ?? []).map((d)=>u.find((m)=>m.column.id === d)).filter(Boolean).map((d)=>({
                        ...d,
                        position: "left"
                    })), P(i.options, "debugRows")), l.getRightVisibleCells = W(()=>[
                    l._getAllVisibleCells(),
                    i.getState().columnPinning.right
                ], (u, r)=>(r ?? []).map((d)=>u.find((m)=>m.column.id === d)).filter(Boolean).map((d)=>({
                        ...d,
                        position: "right"
                    })), P(i.options, "debugRows"));
        },
        createTable: (l)=>{
            l.setColumnPinning = (i)=>l.options.onColumnPinningChange == null ? void 0 : l.options.onColumnPinningChange(i), l.resetColumnPinning = (i)=>{
                var u, r;
                return l.setColumnPinning(i ? Pr() : (u = (r = l.initialState) == null ? void 0 : r.columnPinning) != null ? u : Pr());
            }, l.getIsSomeColumnsPinned = (i)=>{
                var u;
                const r = l.getState().columnPinning;
                if (!i) {
                    var c, d;
                    return !!((c = r.left) != null && c.length || (d = r.right) != null && d.length);
                }
                return !!((u = r[i]) != null && u.length);
            }, l.getLeftLeafColumns = W(()=>[
                    l.getAllLeafColumns(),
                    l.getState().columnPinning.left
                ], (i, u)=>(u ?? []).map((r)=>i.find((c)=>c.id === r)).filter(Boolean), P(l.options, "debugColumns")), l.getRightLeafColumns = W(()=>[
                    l.getAllLeafColumns(),
                    l.getState().columnPinning.right
                ], (i, u)=>(u ?? []).map((r)=>i.find((c)=>c.id === r)).filter(Boolean), P(l.options, "debugColumns")), l.getCenterLeafColumns = W(()=>[
                    l.getAllLeafColumns(),
                    l.getState().columnPinning.left,
                    l.getState().columnPinning.right
                ], (i, u, r)=>{
                const c = [
                    ...u ?? [],
                    ...r ?? []
                ];
                return i.filter((d)=>!c.includes(d.id));
            }, P(l.options, "debugColumns"));
        }
    };
    function bv(l) {
        return l || (typeof document < "u" ? document : null);
    }
    const xu = {
        size: 150,
        minSize: 20,
        maxSize: Number.MAX_SAFE_INTEGER
    }, Ir = ()=>({
            startOffset: null,
            startSize: null,
            deltaOffset: null,
            deltaPercentage: null,
            isResizingColumn: !1,
            columnSizingStart: []
        }), xv = {
        getDefaultColumnDef: ()=>xu,
        getInitialState: (l)=>({
                columnSizing: {},
                columnSizingInfo: Ir(),
                ...l
            }),
        getDefaultOptions: (l)=>({
                columnResizeMode: "onEnd",
                columnResizeDirection: "ltr",
                onColumnSizingChange: bt("columnSizing", l),
                onColumnSizingInfoChange: bt("columnSizingInfo", l)
            }),
        createColumn: (l, i)=>{
            l.getSize = ()=>{
                var u, r, c;
                const d = i.getState().columnSizing[l.id];
                return Math.min(Math.max((u = l.columnDef.minSize) != null ? u : xu.minSize, (r = d ?? l.columnDef.size) != null ? r : xu.size), (c = l.columnDef.maxSize) != null ? c : xu.maxSize);
            }, l.getStart = W((u)=>[
                    u,
                    ka(i, u),
                    i.getState().columnSizing
                ], (u, r)=>r.slice(0, l.getIndex(u)).reduce((c, d)=>c + d.getSize(), 0), P(i.options, "debugColumns")), l.getAfter = W((u)=>[
                    u,
                    ka(i, u),
                    i.getState().columnSizing
                ], (u, r)=>r.slice(l.getIndex(u) + 1).reduce((c, d)=>c + d.getSize(), 0), P(i.options, "debugColumns")), l.resetSize = ()=>{
                i.setColumnSizing((u)=>{
                    let { [l.id]: r, ...c } = u;
                    return c;
                });
            }, l.getCanResize = ()=>{
                var u, r;
                return ((u = l.columnDef.enableResizing) != null ? u : !0) && ((r = i.options.enableColumnResizing) != null ? r : !0);
            }, l.getIsResizing = ()=>i.getState().columnSizingInfo.isResizingColumn === l.id;
        },
        createHeader: (l, i)=>{
            l.getSize = ()=>{
                let u = 0;
                const r = (c)=>{
                    if (c.subHeaders.length) c.subHeaders.forEach(r);
                    else {
                        var d;
                        u += (d = c.column.getSize()) != null ? d : 0;
                    }
                };
                return r(l), u;
            }, l.getStart = ()=>{
                if (l.index > 0) {
                    const u = l.headerGroup.headers[l.index - 1];
                    return u.getStart() + u.getSize();
                }
                return 0;
            }, l.getResizeHandler = (u)=>{
                const r = i.getColumn(l.column.id), c = r?.getCanResize();
                return (d)=>{
                    if (!r || !c || (d.persist == null || d.persist(), eo(d) && d.touches && d.touches.length > 1)) return;
                    const m = l.getSize(), v = l ? l.getLeafHeaders().map((G)=>[
                            G.column.id,
                            G.column.getSize()
                        ]) : [
                        [
                            r.id,
                            r.getSize()
                        ]
                    ], y = eo(d) ? Math.round(d.touches[0].clientX) : d.clientX, p = {}, _ = (G, ae)=>{
                        typeof ae == "number" && (i.setColumnSizingInfo((Z)=>{
                            var oe, he;
                            const F = i.options.columnResizeDirection === "rtl" ? -1 : 1, $ = (ae - ((oe = Z?.startOffset) != null ? oe : 0)) * F, I = Math.max($ / ((he = Z?.startSize) != null ? he : 0), -.999999);
                            return Z.columnSizingStart.forEach((Me)=>{
                                let [Pe, Be] = Me;
                                p[Pe] = Math.round(Math.max(Be + Be * I, 0) * 100) / 100;
                            }), {
                                ...Z,
                                deltaOffset: $,
                                deltaPercentage: I
                            };
                        }), (i.options.columnResizeMode === "onChange" || G === "end") && i.setColumnSizing((Z)=>({
                                ...Z,
                                ...p
                            })));
                    }, M = (G)=>_("move", G), j = (G)=>{
                        _("end", G), i.setColumnSizingInfo((ae)=>({
                                ...ae,
                                isResizingColumn: !1,
                                startOffset: null,
                                startSize: null,
                                deltaOffset: null,
                                deltaPercentage: null,
                                columnSizingStart: []
                            }));
                    }, T = bv(u), U = {
                        moveHandler: (G)=>M(G.clientX),
                        upHandler: (G)=>{
                            T?.removeEventListener("mousemove", U.moveHandler), T?.removeEventListener("mouseup", U.upHandler), j(G.clientX);
                        }
                    }, q = {
                        moveHandler: (G)=>(G.cancelable && (G.preventDefault(), G.stopPropagation()), M(G.touches[0].clientX), !1),
                        upHandler: (G)=>{
                            var ae;
                            T?.removeEventListener("touchmove", q.moveHandler), T?.removeEventListener("touchend", q.upHandler), G.cancelable && (G.preventDefault(), G.stopPropagation()), j((ae = G.touches[0]) == null ? void 0 : ae.clientX);
                        }
                    }, B = _v() ? {
                        passive: !1
                    } : !1;
                    eo(d) ? (T?.addEventListener("touchmove", q.moveHandler, B), T?.addEventListener("touchend", q.upHandler, B)) : (T?.addEventListener("mousemove", U.moveHandler, B), T?.addEventListener("mouseup", U.upHandler, B)), i.setColumnSizingInfo((G)=>({
                            ...G,
                            startOffset: y,
                            startSize: m,
                            deltaOffset: 0,
                            deltaPercentage: 0,
                            columnSizingStart: v,
                            isResizingColumn: r.id
                        }));
                };
            };
        },
        createTable: (l)=>{
            l.setColumnSizing = (i)=>l.options.onColumnSizingChange == null ? void 0 : l.options.onColumnSizingChange(i), l.setColumnSizingInfo = (i)=>l.options.onColumnSizingInfoChange == null ? void 0 : l.options.onColumnSizingInfoChange(i), l.resetColumnSizing = (i)=>{
                var u;
                l.setColumnSizing(i ? {} : (u = l.initialState.columnSizing) != null ? u : {});
            }, l.resetHeaderSizeInfo = (i)=>{
                var u;
                l.setColumnSizingInfo(i ? Ir() : (u = l.initialState.columnSizingInfo) != null ? u : Ir());
            }, l.getTotalSize = ()=>{
                var i, u;
                return (i = (u = l.getHeaderGroups()[0]) == null ? void 0 : u.headers.reduce((r, c)=>r + c.getSize(), 0)) != null ? i : 0;
            }, l.getLeftTotalSize = ()=>{
                var i, u;
                return (i = (u = l.getLeftHeaderGroups()[0]) == null ? void 0 : u.headers.reduce((r, c)=>r + c.getSize(), 0)) != null ? i : 0;
            }, l.getCenterTotalSize = ()=>{
                var i, u;
                return (i = (u = l.getCenterHeaderGroups()[0]) == null ? void 0 : u.headers.reduce((r, c)=>r + c.getSize(), 0)) != null ? i : 0;
            }, l.getRightTotalSize = ()=>{
                var i, u;
                return (i = (u = l.getRightHeaderGroups()[0]) == null ? void 0 : u.headers.reduce((r, c)=>r + c.getSize(), 0)) != null ? i : 0;
            };
        }
    };
    let _u = null;
    function _v() {
        if (typeof _u == "boolean") return _u;
        let l = !1;
        try {
            const i = {
                get passive () {
                    return l = !0, !1;
                }
            }, u = ()=>{};
            window.addEventListener("test", u, i), window.removeEventListener("test", u);
        } catch  {
            l = !1;
        }
        return _u = l, _u;
    }
    function eo(l) {
        return l.type === "touchstart";
    }
    const Cv = {
        getInitialState: (l)=>({
                columnVisibility: {},
                ...l
            }),
        getDefaultOptions: (l)=>({
                onColumnVisibilityChange: bt("columnVisibility", l)
            }),
        createColumn: (l, i)=>{
            l.toggleVisibility = (u)=>{
                l.getCanHide() && i.setColumnVisibility((r)=>({
                        ...r,
                        [l.id]: u ?? !l.getIsVisible()
                    }));
            }, l.getIsVisible = ()=>{
                var u, r;
                const c = l.columns;
                return (u = c.length ? c.some((d)=>d.getIsVisible()) : (r = i.getState().columnVisibility) == null ? void 0 : r[l.id]) != null ? u : !0;
            }, l.getCanHide = ()=>{
                var u, r;
                return ((u = l.columnDef.enableHiding) != null ? u : !0) && ((r = i.options.enableHiding) != null ? r : !0);
            }, l.getToggleVisibilityHandler = ()=>(u)=>{
                    l.toggleVisibility == null || l.toggleVisibility(u.target.checked);
                };
        },
        createRow: (l, i)=>{
            l._getAllVisibleCells = W(()=>[
                    l.getAllCells(),
                    i.getState().columnVisibility
                ], (u)=>u.filter((r)=>r.column.getIsVisible()), P(i.options, "debugRows")), l.getVisibleCells = W(()=>[
                    l.getLeftVisibleCells(),
                    l.getCenterVisibleCells(),
                    l.getRightVisibleCells()
                ], (u, r, c)=>[
                    ...u,
                    ...r,
                    ...c
                ], P(i.options, "debugRows"));
        },
        createTable: (l)=>{
            const i = (u, r)=>W(()=>[
                        r(),
                        r().filter((c)=>c.getIsVisible()).map((c)=>c.id).join("_")
                    ], (c)=>c.filter((d)=>d.getIsVisible == null ? void 0 : d.getIsVisible()), P(l.options, "debugColumns"));
            l.getVisibleFlatColumns = i("getVisibleFlatColumns", ()=>l.getAllFlatColumns()), l.getVisibleLeafColumns = i("getVisibleLeafColumns", ()=>l.getAllLeafColumns()), l.getLeftVisibleLeafColumns = i("getLeftVisibleLeafColumns", ()=>l.getLeftLeafColumns()), l.getRightVisibleLeafColumns = i("getRightVisibleLeafColumns", ()=>l.getRightLeafColumns()), l.getCenterVisibleLeafColumns = i("getCenterVisibleLeafColumns", ()=>l.getCenterLeafColumns()), l.setColumnVisibility = (u)=>l.options.onColumnVisibilityChange == null ? void 0 : l.options.onColumnVisibilityChange(u), l.resetColumnVisibility = (u)=>{
                var r;
                l.setColumnVisibility(u ? {} : (r = l.initialState.columnVisibility) != null ? r : {});
            }, l.toggleAllColumnsVisible = (u)=>{
                var r;
                u = (r = u) != null ? r : !l.getIsAllColumnsVisible(), l.setColumnVisibility(l.getAllLeafColumns().reduce((c, d)=>({
                        ...c,
                        [d.id]: u || !(d.getCanHide != null && d.getCanHide())
                    }), {}));
            }, l.getIsAllColumnsVisible = ()=>!l.getAllLeafColumns().some((u)=>!(u.getIsVisible != null && u.getIsVisible())), l.getIsSomeColumnsVisible = ()=>l.getAllLeafColumns().some((u)=>u.getIsVisible == null ? void 0 : u.getIsVisible()), l.getToggleAllColumnsVisibilityHandler = ()=>(u)=>{
                    var r;
                    l.toggleAllColumnsVisible((r = u.target) == null ? void 0 : r.checked);
                };
        }
    };
    function ka(l, i) {
        return i ? i === "center" ? l.getCenterVisibleLeafColumns() : i === "left" ? l.getLeftVisibleLeafColumns() : l.getRightVisibleLeafColumns() : l.getVisibleLeafColumns();
    }
    const Rv = {
        createTable: (l)=>{
            l._getGlobalFacetedRowModel = l.options.getFacetedRowModel && l.options.getFacetedRowModel(l, "__global__"), l.getGlobalFacetedRowModel = ()=>l.options.manualFiltering || !l._getGlobalFacetedRowModel ? l.getPreFilteredRowModel() : l._getGlobalFacetedRowModel(), l._getGlobalFacetedUniqueValues = l.options.getFacetedUniqueValues && l.options.getFacetedUniqueValues(l, "__global__"), l.getGlobalFacetedUniqueValues = ()=>l._getGlobalFacetedUniqueValues ? l._getGlobalFacetedUniqueValues() : new Map, l._getGlobalFacetedMinMaxValues = l.options.getFacetedMinMaxValues && l.options.getFacetedMinMaxValues(l, "__global__"), l.getGlobalFacetedMinMaxValues = ()=>{
                if (l._getGlobalFacetedMinMaxValues) return l._getGlobalFacetedMinMaxValues();
            };
        }
    }, wv = {
        getInitialState: (l)=>({
                globalFilter: void 0,
                ...l
            }),
        getDefaultOptions: (l)=>({
                onGlobalFilterChange: bt("globalFilter", l),
                globalFilterFn: "auto",
                getColumnCanGlobalFilter: (i)=>{
                    var u;
                    const r = (u = l.getCoreRowModel().flatRows[0]) == null || (u = u._getAllCellsByColumnId()[i.id]) == null ? void 0 : u.getValue();
                    return typeof r == "string" || typeof r == "number";
                }
            }),
        createColumn: (l, i)=>{
            l.getCanGlobalFilter = ()=>{
                var u, r, c, d;
                return ((u = l.columnDef.enableGlobalFilter) != null ? u : !0) && ((r = i.options.enableGlobalFilter) != null ? r : !0) && ((c = i.options.enableFilters) != null ? c : !0) && ((d = i.options.getColumnCanGlobalFilter == null ? void 0 : i.options.getColumnCanGlobalFilter(l)) != null ? d : !0) && !!l.accessorFn;
            };
        },
        createTable: (l)=>{
            l.getGlobalAutoFilterFn = ()=>rn.includesString, l.getGlobalFilterFn = ()=>{
                var i, u;
                const { globalFilterFn: r } = l.options;
                return wu(r) ? r : r === "auto" ? l.getGlobalAutoFilterFn() : (i = (u = l.options.filterFns) == null ? void 0 : u[r]) != null ? i : rn[r];
            }, l.setGlobalFilter = (i)=>{
                l.options.onGlobalFilterChange == null || l.options.onGlobalFilterChange(i);
            }, l.resetGlobalFilter = (i)=>{
                l.setGlobalFilter(i ? void 0 : l.initialState.globalFilter);
            };
        }
    }, Ev = {
        getInitialState: (l)=>({
                expanded: {},
                ...l
            }),
        getDefaultOptions: (l)=>({
                onExpandedChange: bt("expanded", l),
                paginateExpandedRows: !0
            }),
        createTable: (l)=>{
            let i = !1, u = !1;
            l._autoResetExpanded = ()=>{
                var r, c;
                if (!i) {
                    l._queue(()=>{
                        i = !0;
                    });
                    return;
                }
                if ((r = (c = l.options.autoResetAll) != null ? c : l.options.autoResetExpanded) != null ? r : !l.options.manualExpanding) {
                    if (u) return;
                    u = !0, l._queue(()=>{
                        l.resetExpanded(), u = !1;
                    });
                }
            }, l.setExpanded = (r)=>l.options.onExpandedChange == null ? void 0 : l.options.onExpandedChange(r), l.toggleAllRowsExpanded = (r)=>{
                r ?? !l.getIsAllRowsExpanded() ? l.setExpanded(!0) : l.setExpanded({});
            }, l.resetExpanded = (r)=>{
                var c, d;
                l.setExpanded(r ? {} : (c = (d = l.initialState) == null ? void 0 : d.expanded) != null ? c : {});
            }, l.getCanSomeRowsExpand = ()=>l.getPrePaginationRowModel().flatRows.some((r)=>r.getCanExpand()), l.getToggleAllRowsExpandedHandler = ()=>(r)=>{
                    r.persist == null || r.persist(), l.toggleAllRowsExpanded();
                }, l.getIsSomeRowsExpanded = ()=>{
                const r = l.getState().expanded;
                return r === !0 || Object.values(r).some(Boolean);
            }, l.getIsAllRowsExpanded = ()=>{
                const r = l.getState().expanded;
                return typeof r == "boolean" ? r === !0 : !(!Object.keys(r).length || l.getRowModel().flatRows.some((c)=>!c.getIsExpanded()));
            }, l.getExpandedDepth = ()=>{
                let r = 0;
                return (l.getState().expanded === !0 ? Object.keys(l.getRowModel().rowsById) : Object.keys(l.getState().expanded)).forEach((d)=>{
                    const m = d.split(".");
                    r = Math.max(r, m.length);
                }), r;
            }, l.getPreExpandedRowModel = ()=>l.getSortedRowModel(), l.getExpandedRowModel = ()=>(!l._getExpandedRowModel && l.options.getExpandedRowModel && (l._getExpandedRowModel = l.options.getExpandedRowModel(l)), l.options.manualExpanding || !l._getExpandedRowModel ? l.getPreExpandedRowModel() : l._getExpandedRowModel());
        },
        createRow: (l, i)=>{
            l.toggleExpanded = (u)=>{
                i.setExpanded((r)=>{
                    var c;
                    const d = r === !0 ? !0 : !!(r != null && r[l.id]);
                    let m = {};
                    if (r === !0 ? Object.keys(i.getRowModel().rowsById).forEach((v)=>{
                        m[v] = !0;
                    }) : m = r, u = (c = u) != null ? c : !d, !d && u) return {
                        ...m,
                        [l.id]: !0
                    };
                    if (d && !u) {
                        const { [l.id]: v, ...y } = m;
                        return y;
                    }
                    return r;
                });
            }, l.getIsExpanded = ()=>{
                var u;
                const r = i.getState().expanded;
                return !!((u = i.options.getIsRowExpanded == null ? void 0 : i.options.getIsRowExpanded(l)) != null ? u : r === !0 || r?.[l.id]);
            }, l.getCanExpand = ()=>{
                var u, r, c;
                return (u = i.options.getRowCanExpand == null ? void 0 : i.options.getRowCanExpand(l)) != null ? u : ((r = i.options.enableExpanding) != null ? r : !0) && !!((c = l.subRows) != null && c.length);
            }, l.getIsAllParentsExpanded = ()=>{
                let u = !0, r = l;
                for(; u && r.parentId;)r = i.getRow(r.parentId, !0), u = r.getIsExpanded();
                return u;
            }, l.getToggleExpandedHandler = ()=>{
                const u = l.getCanExpand();
                return ()=>{
                    u && l.toggleExpanded();
                };
            };
        }
    }, go = 0, mo = 10, to = ()=>({
            pageIndex: go,
            pageSize: mo
        }), Mv = {
        getInitialState: (l)=>({
                ...l,
                pagination: {
                    ...to(),
                    ...l?.pagination
                }
            }),
        getDefaultOptions: (l)=>({
                onPaginationChange: bt("pagination", l)
            }),
        createTable: (l)=>{
            let i = !1, u = !1;
            l._autoResetPageIndex = ()=>{
                var r, c;
                if (!i) {
                    l._queue(()=>{
                        i = !0;
                    });
                    return;
                }
                if ((r = (c = l.options.autoResetAll) != null ? c : l.options.autoResetPageIndex) != null ? r : !l.options.manualPagination) {
                    if (u) return;
                    u = !0, l._queue(()=>{
                        l.resetPageIndex(), u = !1;
                    });
                }
            }, l.setPagination = (r)=>{
                const c = (d)=>Hn(r, d);
                return l.options.onPaginationChange == null ? void 0 : l.options.onPaginationChange(c);
            }, l.resetPagination = (r)=>{
                var c;
                l.setPagination(r ? to() : (c = l.initialState.pagination) != null ? c : to());
            }, l.setPageIndex = (r)=>{
                l.setPagination((c)=>{
                    let d = Hn(r, c.pageIndex);
                    const m = typeof l.options.pageCount > "u" || l.options.pageCount === -1 ? Number.MAX_SAFE_INTEGER : l.options.pageCount - 1;
                    return d = Math.max(0, Math.min(d, m)), {
                        ...c,
                        pageIndex: d
                    };
                });
            }, l.resetPageIndex = (r)=>{
                var c, d;
                l.setPageIndex(r ? go : (c = (d = l.initialState) == null || (d = d.pagination) == null ? void 0 : d.pageIndex) != null ? c : go);
            }, l.resetPageSize = (r)=>{
                var c, d;
                l.setPageSize(r ? mo : (c = (d = l.initialState) == null || (d = d.pagination) == null ? void 0 : d.pageSize) != null ? c : mo);
            }, l.setPageSize = (r)=>{
                l.setPagination((c)=>{
                    const d = Math.max(1, Hn(r, c.pageSize)), m = c.pageSize * c.pageIndex, v = Math.floor(m / d);
                    return {
                        ...c,
                        pageIndex: v,
                        pageSize: d
                    };
                });
            }, l.setPageCount = (r)=>l.setPagination((c)=>{
                    var d;
                    let m = Hn(r, (d = l.options.pageCount) != null ? d : -1);
                    return typeof m == "number" && (m = Math.max(-1, m)), {
                        ...c,
                        pageCount: m
                    };
                }), l.getPageOptions = W(()=>[
                    l.getPageCount()
                ], (r)=>{
                let c = [];
                return r && r > 0 && (c = [
                    ...new Array(r)
                ].fill(null).map((d, m)=>m)), c;
            }, P(l.options, "debugTable")), l.getCanPreviousPage = ()=>l.getState().pagination.pageIndex > 0, l.getCanNextPage = ()=>{
                const { pageIndex: r } = l.getState().pagination, c = l.getPageCount();
                return c === -1 ? !0 : c === 0 ? !1 : r < c - 1;
            }, l.previousPage = ()=>l.setPageIndex((r)=>r - 1), l.nextPage = ()=>l.setPageIndex((r)=>r + 1), l.firstPage = ()=>l.setPageIndex(0), l.lastPage = ()=>l.setPageIndex(l.getPageCount() - 1), l.getPrePaginationRowModel = ()=>l.getExpandedRowModel(), l.getPaginationRowModel = ()=>(!l._getPaginationRowModel && l.options.getPaginationRowModel && (l._getPaginationRowModel = l.options.getPaginationRowModel(l)), l.options.manualPagination || !l._getPaginationRowModel ? l.getPrePaginationRowModel() : l._getPaginationRowModel()), l.getPageCount = ()=>{
                var r;
                return (r = l.options.pageCount) != null ? r : Math.ceil(l.getRowCount() / l.getState().pagination.pageSize);
            }, l.getRowCount = ()=>{
                var r;
                return (r = l.options.rowCount) != null ? r : l.getPrePaginationRowModel().rows.length;
            };
        }
    }, no = ()=>({
            top: [],
            bottom: []
        }), jv = {
        getInitialState: (l)=>({
                rowPinning: no(),
                ...l
            }),
        getDefaultOptions: (l)=>({
                onRowPinningChange: bt("rowPinning", l)
            }),
        createRow: (l, i)=>{
            l.pin = (u, r, c)=>{
                const d = r ? l.getLeafRows().map((y)=>{
                    let { id: p } = y;
                    return p;
                }) : [], m = c ? l.getParentRows().map((y)=>{
                    let { id: p } = y;
                    return p;
                }) : [], v = new Set([
                    ...m,
                    l.id,
                    ...d
                ]);
                i.setRowPinning((y)=>{
                    var p, _;
                    if (u === "bottom") {
                        var M, j;
                        return {
                            top: ((M = y?.top) != null ? M : []).filter((q)=>!(v != null && v.has(q))),
                            bottom: [
                                ...((j = y?.bottom) != null ? j : []).filter((q)=>!(v != null && v.has(q))),
                                ...Array.from(v)
                            ]
                        };
                    }
                    if (u === "top") {
                        var T, U;
                        return {
                            top: [
                                ...((T = y?.top) != null ? T : []).filter((q)=>!(v != null && v.has(q))),
                                ...Array.from(v)
                            ],
                            bottom: ((U = y?.bottom) != null ? U : []).filter((q)=>!(v != null && v.has(q)))
                        };
                    }
                    return {
                        top: ((p = y?.top) != null ? p : []).filter((q)=>!(v != null && v.has(q))),
                        bottom: ((_ = y?.bottom) != null ? _ : []).filter((q)=>!(v != null && v.has(q)))
                    };
                });
            }, l.getCanPin = ()=>{
                var u;
                const { enableRowPinning: r, enablePinning: c } = i.options;
                return typeof r == "function" ? r(l) : (u = r ?? c) != null ? u : !0;
            }, l.getIsPinned = ()=>{
                const u = [
                    l.id
                ], { top: r, bottom: c } = i.getState().rowPinning, d = u.some((v)=>r?.includes(v)), m = u.some((v)=>c?.includes(v));
                return d ? "top" : m ? "bottom" : !1;
            }, l.getPinnedIndex = ()=>{
                var u, r;
                const c = l.getIsPinned();
                if (!c) return -1;
                const d = (u = c === "top" ? i.getTopRows() : i.getBottomRows()) == null ? void 0 : u.map((m)=>{
                    let { id: v } = m;
                    return v;
                });
                return (r = d?.indexOf(l.id)) != null ? r : -1;
            };
        },
        createTable: (l)=>{
            l.setRowPinning = (i)=>l.options.onRowPinningChange == null ? void 0 : l.options.onRowPinningChange(i), l.resetRowPinning = (i)=>{
                var u, r;
                return l.setRowPinning(i ? no() : (u = (r = l.initialState) == null ? void 0 : r.rowPinning) != null ? u : no());
            }, l.getIsSomeRowsPinned = (i)=>{
                var u;
                const r = l.getState().rowPinning;
                if (!i) {
                    var c, d;
                    return !!((c = r.top) != null && c.length || (d = r.bottom) != null && d.length);
                }
                return !!((u = r[i]) != null && u.length);
            }, l._getPinnedRows = (i, u, r)=>{
                var c;
                return ((c = l.options.keepPinnedRows) == null || c ? (u ?? []).map((m)=>{
                    const v = l.getRow(m, !0);
                    return v.getIsAllParentsExpanded() ? v : null;
                }) : (u ?? []).map((m)=>i.find((v)=>v.id === m))).filter(Boolean).map((m)=>({
                        ...m,
                        position: r
                    }));
            }, l.getTopRows = W(()=>[
                    l.getRowModel().rows,
                    l.getState().rowPinning.top
                ], (i, u)=>l._getPinnedRows(i, u, "top"), P(l.options, "debugRows")), l.getBottomRows = W(()=>[
                    l.getRowModel().rows,
                    l.getState().rowPinning.bottom
                ], (i, u)=>l._getPinnedRows(i, u, "bottom"), P(l.options, "debugRows")), l.getCenterRows = W(()=>[
                    l.getRowModel().rows,
                    l.getState().rowPinning.top,
                    l.getState().rowPinning.bottom
                ], (i, u, r)=>{
                const c = new Set([
                    ...u ?? [],
                    ...r ?? []
                ]);
                return i.filter((d)=>!c.has(d.id));
            }, P(l.options, "debugRows"));
        }
    }, Ov = {
        getInitialState: (l)=>({
                rowSelection: {},
                ...l
            }),
        getDefaultOptions: (l)=>({
                onRowSelectionChange: bt("rowSelection", l),
                enableRowSelection: !0,
                enableMultiRowSelection: !0,
                enableSubRowSelection: !0
            }),
        createTable: (l)=>{
            l.setRowSelection = (i)=>l.options.onRowSelectionChange == null ? void 0 : l.options.onRowSelectionChange(i), l.resetRowSelection = (i)=>{
                var u;
                return l.setRowSelection(i ? {} : (u = l.initialState.rowSelection) != null ? u : {});
            }, l.toggleAllRowsSelected = (i)=>{
                l.setRowSelection((u)=>{
                    i = typeof i < "u" ? i : !l.getIsAllRowsSelected();
                    const r = {
                        ...u
                    }, c = l.getPreGroupedRowModel().flatRows;
                    return i ? c.forEach((d)=>{
                        d.getCanSelect() && (r[d.id] = !0);
                    }) : c.forEach((d)=>{
                        delete r[d.id];
                    }), r;
                });
            }, l.toggleAllPageRowsSelected = (i)=>l.setRowSelection((u)=>{
                    const r = typeof i < "u" ? i : !l.getIsAllPageRowsSelected(), c = {
                        ...u
                    };
                    return l.getRowModel().rows.forEach((d)=>{
                        yo(c, d.id, r, !0, l);
                    }), c;
                }), l.getPreSelectedRowModel = ()=>l.getCoreRowModel(), l.getSelectedRowModel = W(()=>[
                    l.getState().rowSelection,
                    l.getCoreRowModel()
                ], (i, u)=>Object.keys(i).length ? lo(l, u) : {
                    rows: [],
                    flatRows: [],
                    rowsById: {}
                }, P(l.options, "debugTable")), l.getFilteredSelectedRowModel = W(()=>[
                    l.getState().rowSelection,
                    l.getFilteredRowModel()
                ], (i, u)=>Object.keys(i).length ? lo(l, u) : {
                    rows: [],
                    flatRows: [],
                    rowsById: {}
                }, P(l.options, "debugTable")), l.getGroupedSelectedRowModel = W(()=>[
                    l.getState().rowSelection,
                    l.getSortedRowModel()
                ], (i, u)=>Object.keys(i).length ? lo(l, u) : {
                    rows: [],
                    flatRows: [],
                    rowsById: {}
                }, P(l.options, "debugTable")), l.getIsAllRowsSelected = ()=>{
                const i = l.getFilteredRowModel().flatRows, { rowSelection: u } = l.getState();
                let r = !!(i.length && Object.keys(u).length);
                return r && i.some((c)=>c.getCanSelect() && !u[c.id]) && (r = !1), r;
            }, l.getIsAllPageRowsSelected = ()=>{
                const i = l.getPaginationRowModel().flatRows.filter((c)=>c.getCanSelect()), { rowSelection: u } = l.getState();
                let r = !!i.length;
                return r && i.some((c)=>!u[c.id]) && (r = !1), r;
            }, l.getIsSomeRowsSelected = ()=>{
                var i;
                const u = Object.keys((i = l.getState().rowSelection) != null ? i : {}).length;
                return u > 0 && u < l.getFilteredRowModel().flatRows.length;
            }, l.getIsSomePageRowsSelected = ()=>{
                const i = l.getPaginationRowModel().flatRows;
                return l.getIsAllPageRowsSelected() ? !1 : i.filter((u)=>u.getCanSelect()).some((u)=>u.getIsSelected() || u.getIsSomeSelected());
            }, l.getToggleAllRowsSelectedHandler = ()=>(i)=>{
                    l.toggleAllRowsSelected(i.target.checked);
                }, l.getToggleAllPageRowsSelectedHandler = ()=>(i)=>{
                    l.toggleAllPageRowsSelected(i.target.checked);
                };
        },
        createRow: (l, i)=>{
            l.toggleSelected = (u, r)=>{
                const c = l.getIsSelected();
                i.setRowSelection((d)=>{
                    var m;
                    if (u = typeof u < "u" ? u : !c, l.getCanSelect() && c === u) return d;
                    const v = {
                        ...d
                    };
                    return yo(v, l.id, u, (m = r?.selectChildren) != null ? m : !0, i), v;
                });
            }, l.getIsSelected = ()=>{
                const { rowSelection: u } = i.getState();
                return wo(l, u);
            }, l.getIsSomeSelected = ()=>{
                const { rowSelection: u } = i.getState();
                return vo(l, u) === "some";
            }, l.getIsAllSubRowsSelected = ()=>{
                const { rowSelection: u } = i.getState();
                return vo(l, u) === "all";
            }, l.getCanSelect = ()=>{
                var u;
                return typeof i.options.enableRowSelection == "function" ? i.options.enableRowSelection(l) : (u = i.options.enableRowSelection) != null ? u : !0;
            }, l.getCanSelectSubRows = ()=>{
                var u;
                return typeof i.options.enableSubRowSelection == "function" ? i.options.enableSubRowSelection(l) : (u = i.options.enableSubRowSelection) != null ? u : !0;
            }, l.getCanMultiSelect = ()=>{
                var u;
                return typeof i.options.enableMultiRowSelection == "function" ? i.options.enableMultiRowSelection(l) : (u = i.options.enableMultiRowSelection) != null ? u : !0;
            }, l.getToggleSelectedHandler = ()=>{
                const u = l.getCanSelect();
                return (r)=>{
                    var c;
                    u && l.toggleSelected((c = r.target) == null ? void 0 : c.checked);
                };
            };
        }
    }, yo = (l, i, u, r, c)=>{
        var d;
        const m = c.getRow(i, !0);
        u ? (m.getCanMultiSelect() || Object.keys(l).forEach((v)=>delete l[v]), m.getCanSelect() && (l[i] = !0)) : delete l[i], r && (d = m.subRows) != null && d.length && m.getCanSelectSubRows() && m.subRows.forEach((v)=>yo(l, v.id, u, r, c));
    };
    function lo(l, i) {
        const u = l.getState().rowSelection, r = [], c = {}, d = function(m, v) {
            return m.map((y)=>{
                var p;
                const _ = wo(y, u);
                if (_ && (r.push(y), c[y.id] = y), (p = y.subRows) != null && p.length && (y = {
                    ...y,
                    subRows: d(y.subRows)
                }), _) return y;
            }).filter(Boolean);
        };
        return {
            rows: d(i.rows),
            flatRows: r,
            rowsById: c
        };
    }
    function wo(l, i) {
        var u;
        return (u = i[l.id]) != null ? u : !1;
    }
    function vo(l, i, u) {
        var r;
        if (!((r = l.subRows) != null && r.length)) return !1;
        let c = !0, d = !1;
        return l.subRows.forEach((m)=>{
            if (!(d && !c) && (m.getCanSelect() && (wo(m, i) ? d = !0 : c = !1), m.subRows && m.subRows.length)) {
                const v = vo(m, i);
                v === "all" ? d = !0 : (v === "some" && (d = !0), c = !1);
            }
        }), c ? "all" : d ? "some" : !1;
    }
    const po = /([0-9]+)/gm, Tv = (l, i, u)=>Tg(Un(l.getValue(u)).toLowerCase(), Un(i.getValue(u)).toLowerCase()), Nv = (l, i, u)=>Tg(Un(l.getValue(u)), Un(i.getValue(u))), Av = (l, i, u)=>Eo(Un(l.getValue(u)).toLowerCase(), Un(i.getValue(u)).toLowerCase()), zv = (l, i, u)=>Eo(Un(l.getValue(u)), Un(i.getValue(u))), Dv = (l, i, u)=>{
        const r = l.getValue(u), c = i.getValue(u);
        return r > c ? 1 : r < c ? -1 : 0;
    }, Hv = (l, i, u)=>Eo(l.getValue(u), i.getValue(u));
    function Eo(l, i) {
        return l === i ? 0 : l > i ? 1 : -1;
    }
    function Un(l) {
        return typeof l == "number" ? isNaN(l) || l === 1 / 0 || l === -1 / 0 ? "" : String(l) : typeof l == "string" ? l : "";
    }
    function Tg(l, i) {
        const u = l.split(po).filter(Boolean), r = i.split(po).filter(Boolean);
        for(; u.length && r.length;){
            const c = u.shift(), d = r.shift(), m = parseInt(c, 10), v = parseInt(d, 10), y = [
                m,
                v
            ].sort();
            if (isNaN(y[0])) {
                if (c > d) return 1;
                if (d > c) return -1;
                continue;
            }
            if (isNaN(y[1])) return isNaN(m) ? -1 : 1;
            if (m > v) return 1;
            if (v > m) return -1;
        }
        return u.length - r.length;
    }
    const Za = {
        alphanumeric: Tv,
        alphanumericCaseSensitive: Nv,
        text: Av,
        textCaseSensitive: zv,
        datetime: Dv,
        basic: Hv
    }, qv = {
        getInitialState: (l)=>({
                sorting: [],
                ...l
            }),
        getDefaultColumnDef: ()=>({
                sortingFn: "auto",
                sortUndefined: 1
            }),
        getDefaultOptions: (l)=>({
                onSortingChange: bt("sorting", l),
                isMultiSortEvent: (i)=>i.shiftKey
            }),
        createColumn: (l, i)=>{
            l.getAutoSortingFn = ()=>{
                const u = i.getFilteredRowModel().flatRows.slice(10);
                let r = !1;
                for (const c of u){
                    const d = c?.getValue(l.id);
                    if (Object.prototype.toString.call(d) === "[object Date]") return Za.datetime;
                    if (typeof d == "string" && (r = !0, d.split(po).length > 1)) return Za.alphanumeric;
                }
                return r ? Za.text : Za.basic;
            }, l.getAutoSortDir = ()=>{
                const u = i.getFilteredRowModel().flatRows[0];
                return typeof u?.getValue(l.id) == "string" ? "asc" : "desc";
            }, l.getSortingFn = ()=>{
                var u, r;
                if (!l) throw new Error;
                return wu(l.columnDef.sortingFn) ? l.columnDef.sortingFn : l.columnDef.sortingFn === "auto" ? l.getAutoSortingFn() : (u = (r = i.options.sortingFns) == null ? void 0 : r[l.columnDef.sortingFn]) != null ? u : Za[l.columnDef.sortingFn];
            }, l.toggleSorting = (u, r)=>{
                const c = l.getNextSortingOrder(), d = typeof u < "u" && u !== null;
                i.setSorting((m)=>{
                    const v = m?.find((T)=>T.id === l.id), y = m?.findIndex((T)=>T.id === l.id);
                    let p = [], _, M = d ? u : c === "desc";
                    if (m != null && m.length && l.getCanMultiSort() && r ? v ? _ = "toggle" : _ = "add" : m != null && m.length && y !== m.length - 1 ? _ = "replace" : v ? _ = "toggle" : _ = "replace", _ === "toggle" && (d || c || (_ = "remove")), _ === "add") {
                        var j;
                        p = [
                            ...m,
                            {
                                id: l.id,
                                desc: M
                            }
                        ], p.splice(0, p.length - ((j = i.options.maxMultiSortColCount) != null ? j : Number.MAX_SAFE_INTEGER));
                    } else _ === "toggle" ? p = m.map((T)=>T.id === l.id ? {
                            ...T,
                            desc: M
                        } : T) : _ === "remove" ? p = m.filter((T)=>T.id !== l.id) : p = [
                        {
                            id: l.id,
                            desc: M
                        }
                    ];
                    return p;
                });
            }, l.getFirstSortDir = ()=>{
                var u, r;
                return ((u = (r = l.columnDef.sortDescFirst) != null ? r : i.options.sortDescFirst) != null ? u : l.getAutoSortDir() === "desc") ? "desc" : "asc";
            }, l.getNextSortingOrder = (u)=>{
                var r, c;
                const d = l.getFirstSortDir(), m = l.getIsSorted();
                return m ? m !== d && ((r = i.options.enableSortingRemoval) == null || r) && (!(u && (c = i.options.enableMultiRemove) != null) || c) ? !1 : m === "desc" ? "asc" : "desc" : d;
            }, l.getCanSort = ()=>{
                var u, r;
                return ((u = l.columnDef.enableSorting) != null ? u : !0) && ((r = i.options.enableSorting) != null ? r : !0) && !!l.accessorFn;
            }, l.getCanMultiSort = ()=>{
                var u, r;
                return (u = (r = l.columnDef.enableMultiSort) != null ? r : i.options.enableMultiSort) != null ? u : !!l.accessorFn;
            }, l.getIsSorted = ()=>{
                var u;
                const r = (u = i.getState().sorting) == null ? void 0 : u.find((c)=>c.id === l.id);
                return r ? r.desc ? "desc" : "asc" : !1;
            }, l.getSortIndex = ()=>{
                var u, r;
                return (u = (r = i.getState().sorting) == null ? void 0 : r.findIndex((c)=>c.id === l.id)) != null ? u : -1;
            }, l.clearSorting = ()=>{
                i.setSorting((u)=>u != null && u.length ? u.filter((r)=>r.id !== l.id) : []);
            }, l.getToggleSortingHandler = ()=>{
                const u = l.getCanSort();
                return (r)=>{
                    u && (r.persist == null || r.persist(), l.toggleSorting == null || l.toggleSorting(void 0, l.getCanMultiSort() ? i.options.isMultiSortEvent == null ? void 0 : i.options.isMultiSortEvent(r) : !1));
                };
            };
        },
        createTable: (l)=>{
            l.setSorting = (i)=>l.options.onSortingChange == null ? void 0 : l.options.onSortingChange(i), l.resetSorting = (i)=>{
                var u, r;
                l.setSorting(i ? [] : (u = (r = l.initialState) == null ? void 0 : r.sorting) != null ? u : []);
            }, l.getPreSortedRowModel = ()=>l.getGroupedRowModel(), l.getSortedRowModel = ()=>(!l._getSortedRowModel && l.options.getSortedRowModel && (l._getSortedRowModel = l.options.getSortedRowModel(l)), l.options.manualSorting || !l._getSortedRowModel ? l.getPreSortedRowModel() : l._getSortedRowModel());
        }
    }, Uv = [
        lv,
        Cv,
        pv,
        Sv,
        iv,
        uv,
        Rv,
        wv,
        qv,
        yv,
        Ev,
        Mv,
        jv,
        Ov,
        xv
    ];
    function Vv(l) {
        var i, u;
        const r = [
            ...Uv,
            ...(i = l._features) != null ? i : []
        ];
        let c = {
            _features: r
        };
        const d = c._features.reduce((j, T)=>Object.assign(j, T.getDefaultOptions == null ? void 0 : T.getDefaultOptions(c)), {}), m = (j)=>c.options.mergeOptions ? c.options.mergeOptions(d, j) : {
                ...d,
                ...j
            };
        let y = {
            ...{},
            ...(u = l.initialState) != null ? u : {}
        };
        c._features.forEach((j)=>{
            var T;
            y = (T = j.getInitialState == null ? void 0 : j.getInitialState(y)) != null ? T : y;
        });
        const p = [];
        let _ = !1;
        const M = {
            _features: r,
            options: {
                ...d,
                ...l
            },
            initialState: y,
            _queue: (j)=>{
                p.push(j), _ || (_ = !0, Promise.resolve().then(()=>{
                    for(; p.length;)p.shift()();
                    _ = !1;
                }).catch((T)=>setTimeout(()=>{
                        throw T;
                    })));
            },
            reset: ()=>{
                c.setState(c.initialState);
            },
            setOptions: (j)=>{
                const T = Hn(j, c.options);
                c.options = m(T);
            },
            getState: ()=>c.options.state,
            setState: (j)=>{
                c.options.onStateChange == null || c.options.onStateChange(j);
            },
            _getRowId: (j, T, U)=>{
                var q;
                return (q = c.options.getRowId == null ? void 0 : c.options.getRowId(j, T, U)) != null ? q : `${U ? [
                    U.id,
                    T
                ].join(".") : T}`;
            },
            getCoreRowModel: ()=>(c._getCoreRowModel || (c._getCoreRowModel = c.options.getCoreRowModel(c)), c._getCoreRowModel()),
            getRowModel: ()=>c.getPaginationRowModel(),
            getRow: (j, T)=>{
                let U = (T ? c.getPrePaginationRowModel() : c.getRowModel()).rowsById[j];
                if (!U && (U = c.getCoreRowModel().rowsById[j], !U)) throw new Error;
                return U;
            },
            _getDefaultColumnDef: W(()=>[
                    c.options.defaultColumn
                ], (j)=>{
                var T;
                return j = (T = j) != null ? T : {}, {
                    header: (U)=>{
                        const q = U.header.column.columnDef;
                        return q.accessorKey ? q.accessorKey : q.accessorFn ? q.id : null;
                    },
                    cell: (U)=>{
                        var q, B;
                        return (q = (B = U.renderValue()) == null || B.toString == null ? void 0 : B.toString()) != null ? q : null;
                    },
                    ...c._features.reduce((U, q)=>Object.assign(U, q.getDefaultColumnDef == null ? void 0 : q.getDefaultColumnDef()), {}),
                    ...j
                };
            }, P(l, "debugColumns")),
            _getColumnDefs: ()=>c.options.columns,
            getAllColumns: W(()=>[
                    c._getColumnDefs()
                ], (j)=>{
                const T = function(U, q, B) {
                    return B === void 0 && (B = 0), U.map((G)=>{
                        const ae = nv(c, G, B, q), Z = G;
                        return ae.columns = Z.columns ? T(Z.columns, ae, B + 1) : [], ae;
                    });
                };
                return T(j);
            }, P(l, "debugColumns")),
            getAllFlatColumns: W(()=>[
                    c.getAllColumns()
                ], (j)=>j.flatMap((T)=>T.getFlatColumns()), P(l, "debugColumns")),
            _getAllFlatColumnsById: W(()=>[
                    c.getAllFlatColumns()
                ], (j)=>j.reduce((T, U)=>(T[U.id] = U, T), {}), P(l, "debugColumns")),
            getAllLeafColumns: W(()=>[
                    c.getAllColumns(),
                    c._getOrderColumnsFn()
                ], (j, T)=>{
                let U = j.flatMap((q)=>q.getLeafColumns());
                return T(U);
            }, P(l, "debugColumns")),
            getColumn: (j)=>c._getAllFlatColumnsById()[j]
        };
        Object.assign(c, M);
        for(let j = 0; j < c._features.length; j++){
            const T = c._features[j];
            T == null || T.createTable == null || T.createTable(c);
        }
        return c;
    }
    function Lv() {
        return (l)=>W(()=>[
                    l.options.data
                ], (i)=>{
                const u = {
                    rows: [],
                    flatRows: [],
                    rowsById: {}
                }, r = function(c, d, m) {
                    d === void 0 && (d = 0);
                    const v = [];
                    for(let p = 0; p < c.length; p++){
                        const _ = av(l, l._getRowId(c[p], p, m), c[p], p, d, void 0, m?.id);
                        if (u.flatRows.push(_), u.rowsById[_.id] = _, v.push(_), l.options.getSubRows) {
                            var y;
                            _.originalSubRows = l.options.getSubRows(c[p], p), (y = _.originalSubRows) != null && y.length && (_.subRows = r(_.originalSubRows, d + 1, _));
                        }
                    }
                    return v;
                };
                return u.rows = r(i), u;
            }, P(l.options, "debugTable", "getRowModel", ()=>l._autoResetPageIndex()));
    }
    function Ph(l, i) {
        return l ? Gv(l) ? ie.createElement(l, i) : l : null;
    }
    function Gv(l) {
        return Bv(l) || typeof l == "function" || Qv(l);
    }
    function Bv(l) {
        return typeof l == "function" && (()=>{
            const i = Object.getPrototypeOf(l);
            return i.prototype && i.prototype.isReactComponent;
        })();
    }
    function Qv(l) {
        return typeof l == "object" && typeof l.$$typeof == "symbol" && [
            "react.memo",
            "react.forward_ref"
        ].includes(l.$$typeof.description);
    }
    function Yv(l) {
        const i = {
            state: {},
            onStateChange: ()=>{},
            renderFallbackValue: null,
            ...l
        }, [u] = ie.useState(()=>({
                current: Vv(i)
            })), [r, c] = ie.useState(()=>u.current.initialState);
        return u.current.setOptions((d)=>({
                ...d,
                ...l,
                state: {
                    ...r,
                    ...l.state
                },
                onStateChange: (m)=>{
                    c(m), l.onStateChange == null || l.onStateChange(m);
                }
            })), u.current;
    }
    const Ng = 6048e5, Xv = 864e5, Ih = Symbol.for("constructDateFrom");
    function Vn(l, i) {
        return typeof l == "function" ? l(i) : l && typeof l == "object" && Ih in l ? l[Ih](i) : l instanceof Date ? new l.constructor(i) : new Date(i);
    }
    function Ut(l, i) {
        return Vn(i || l, l);
    }
    let Fv = {};
    function Eu() {
        return Fv;
    }
    function Ia(l, i) {
        const u = Eu(), r = i?.weekStartsOn ?? i?.locale?.options?.weekStartsOn ?? u.weekStartsOn ?? u.locale?.options?.weekStartsOn ?? 0, c = Ut(l, i?.in), d = c.getDay(), m = (d < r ? 7 : 0) + d - r;
        return c.setDate(c.getDate() - m), c.setHours(0, 0, 0, 0), c;
    }
    function Ru(l, i) {
        return Ia(l, {
            ...i,
            weekStartsOn: 1
        });
    }
    function Ag(l, i) {
        const u = Ut(l, i?.in), r = u.getFullYear(), c = Vn(u, 0);
        c.setFullYear(r + 1, 0, 4), c.setHours(0, 0, 0, 0);
        const d = Ru(c), m = Vn(u, 0);
        m.setFullYear(r, 0, 4), m.setHours(0, 0, 0, 0);
        const v = Ru(m);
        return u.getTime() >= d.getTime() ? r + 1 : u.getTime() >= v.getTime() ? r : r - 1;
    }
    function eg(l) {
        const i = Ut(l), u = new Date(Date.UTC(i.getFullYear(), i.getMonth(), i.getDate(), i.getHours(), i.getMinutes(), i.getSeconds(), i.getMilliseconds()));
        return u.setUTCFullYear(i.getFullYear()), +l - +u;
    }
    function Zv(l, ...i) {
        const u = Vn.bind(null, i.find((r)=>typeof r == "object"));
        return i.map(u);
    }
    function tg(l, i) {
        const u = Ut(l, i?.in);
        return u.setHours(0, 0, 0, 0), u;
    }
    function Kv(l, i, u) {
        const [r, c] = Zv(u?.in, l, i), d = tg(r), m = tg(c), v = +d - eg(d), y = +m - eg(m);
        return Math.round((v - y) / Xv);
    }
    function $v(l, i) {
        const u = Ag(l, i), r = Vn(l, 0);
        return r.setFullYear(u, 0, 4), r.setHours(0, 0, 0, 0), Ru(r);
    }
    function Jv(l) {
        return l instanceof Date || typeof l == "object" && Object.prototype.toString.call(l) === "[object Date]";
    }
    function kv(l) {
        return !(!Jv(l) && typeof l != "number" || isNaN(+Ut(l)));
    }
    function Wv(l, i) {
        const u = Ut(l, i?.in);
        return u.setFullYear(u.getFullYear(), 0, 1), u.setHours(0, 0, 0, 0), u;
    }
    const Pv = {
        lessThanXSeconds: {
            one: "less than a second",
            other: "less than {{count}} seconds"
        },
        xSeconds: {
            one: "1 second",
            other: "{{count}} seconds"
        },
        halfAMinute: "half a minute",
        lessThanXMinutes: {
            one: "less than a minute",
            other: "less than {{count}} minutes"
        },
        xMinutes: {
            one: "1 minute",
            other: "{{count}} minutes"
        },
        aboutXHours: {
            one: "about 1 hour",
            other: "about {{count}} hours"
        },
        xHours: {
            one: "1 hour",
            other: "{{count}} hours"
        },
        xDays: {
            one: "1 day",
            other: "{{count}} days"
        },
        aboutXWeeks: {
            one: "about 1 week",
            other: "about {{count}} weeks"
        },
        xWeeks: {
            one: "1 week",
            other: "{{count}} weeks"
        },
        aboutXMonths: {
            one: "about 1 month",
            other: "about {{count}} months"
        },
        xMonths: {
            one: "1 month",
            other: "{{count}} months"
        },
        aboutXYears: {
            one: "about 1 year",
            other: "about {{count}} years"
        },
        xYears: {
            one: "1 year",
            other: "{{count}} years"
        },
        overXYears: {
            one: "over 1 year",
            other: "over {{count}} years"
        },
        almostXYears: {
            one: "almost 1 year",
            other: "almost {{count}} years"
        }
    }, Iv = (l, i, u)=>{
        let r;
        const c = Pv[l];
        return typeof c == "string" ? r = c : i === 1 ? r = c.one : r = c.other.replace("{{count}}", i.toString()), u?.addSuffix ? u.comparison && u.comparison > 0 ? "in " + r : r + " ago" : r;
    };
    function ao(l) {
        return (i = {})=>{
            const u = i.width ? String(i.width) : l.defaultWidth;
            return l.formats[u] || l.formats[l.defaultWidth];
        };
    }
    const ep = {
        full: "EEEE, MMMM do, y",
        long: "MMMM do, y",
        medium: "MMM d, y",
        short: "MM/dd/yyyy"
    }, tp = {
        full: "h:mm:ss a zzzz",
        long: "h:mm:ss a z",
        medium: "h:mm:ss a",
        short: "h:mm a"
    }, np = {
        full: "{{date}} 'at' {{time}}",
        long: "{{date}} 'at' {{time}}",
        medium: "{{date}}, {{time}}",
        short: "{{date}}, {{time}}"
    }, lp = {
        date: ao({
            formats: ep,
            defaultWidth: "full"
        }),
        time: ao({
            formats: tp,
            defaultWidth: "full"
        }),
        dateTime: ao({
            formats: np,
            defaultWidth: "full"
        })
    }, ap = {
        lastWeek: "'last' eeee 'at' p",
        yesterday: "'yesterday at' p",
        today: "'today at' p",
        tomorrow: "'tomorrow at' p",
        nextWeek: "eeee 'at' p",
        other: "P"
    }, ip = (l, i, u, r)=>ap[l];
    function Ka(l) {
        return (i, u)=>{
            const r = u?.context ? String(u.context) : "standalone";
            let c;
            if (r === "formatting" && l.formattingValues) {
                const m = l.defaultFormattingWidth || l.defaultWidth, v = u?.width ? String(u.width) : m;
                c = l.formattingValues[v] || l.formattingValues[m];
            } else {
                const m = l.defaultWidth, v = u?.width ? String(u.width) : l.defaultWidth;
                c = l.values[v] || l.values[m];
            }
            const d = l.argumentCallback ? l.argumentCallback(i) : i;
            return c[d];
        };
    }
    const up = {
        narrow: [
            "B",
            "A"
        ],
        abbreviated: [
            "BC",
            "AD"
        ],
        wide: [
            "Before Christ",
            "Anno Domini"
        ]
    }, sp = {
        narrow: [
            "1",
            "2",
            "3",
            "4"
        ],
        abbreviated: [
            "Q1",
            "Q2",
            "Q3",
            "Q4"
        ],
        wide: [
            "1st quarter",
            "2nd quarter",
            "3rd quarter",
            "4th quarter"
        ]
    }, rp = {
        narrow: [
            "J",
            "F",
            "M",
            "A",
            "M",
            "J",
            "J",
            "A",
            "S",
            "O",
            "N",
            "D"
        ],
        abbreviated: [
            "Jan",
            "Feb",
            "Mar",
            "Apr",
            "May",
            "Jun",
            "Jul",
            "Aug",
            "Sep",
            "Oct",
            "Nov",
            "Dec"
        ],
        wide: [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December"
        ]
    }, op = {
        narrow: [
            "S",
            "M",
            "T",
            "W",
            "T",
            "F",
            "S"
        ],
        short: [
            "Su",
            "Mo",
            "Tu",
            "We",
            "Th",
            "Fr",
            "Sa"
        ],
        abbreviated: [
            "Sun",
            "Mon",
            "Tue",
            "Wed",
            "Thu",
            "Fri",
            "Sat"
        ],
        wide: [
            "Sunday",
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday"
        ]
    }, cp = {
        narrow: {
            am: "a",
            pm: "p",
            midnight: "mi",
            noon: "n",
            morning: "morning",
            afternoon: "afternoon",
            evening: "evening",
            night: "night"
        },
        abbreviated: {
            am: "AM",
            pm: "PM",
            midnight: "midnight",
            noon: "noon",
            morning: "morning",
            afternoon: "afternoon",
            evening: "evening",
            night: "night"
        },
        wide: {
            am: "a.m.",
            pm: "p.m.",
            midnight: "midnight",
            noon: "noon",
            morning: "morning",
            afternoon: "afternoon",
            evening: "evening",
            night: "night"
        }
    }, fp = {
        narrow: {
            am: "a",
            pm: "p",
            midnight: "mi",
            noon: "n",
            morning: "in the morning",
            afternoon: "in the afternoon",
            evening: "in the evening",
            night: "at night"
        },
        abbreviated: {
            am: "AM",
            pm: "PM",
            midnight: "midnight",
            noon: "noon",
            morning: "in the morning",
            afternoon: "in the afternoon",
            evening: "in the evening",
            night: "at night"
        },
        wide: {
            am: "a.m.",
            pm: "p.m.",
            midnight: "midnight",
            noon: "noon",
            morning: "in the morning",
            afternoon: "in the afternoon",
            evening: "in the evening",
            night: "at night"
        }
    }, dp = (l, i)=>{
        const u = Number(l), r = u % 100;
        if (r > 20 || r < 10) switch(r % 10){
            case 1:
                return u + "st";
            case 2:
                return u + "nd";
            case 3:
                return u + "rd";
        }
        return u + "th";
    }, hp = {
        ordinalNumber: dp,
        era: Ka({
            values: up,
            defaultWidth: "wide"
        }),
        quarter: Ka({
            values: sp,
            defaultWidth: "wide",
            argumentCallback: (l)=>l - 1
        }),
        month: Ka({
            values: rp,
            defaultWidth: "wide"
        }),
        day: Ka({
            values: op,
            defaultWidth: "wide"
        }),
        dayPeriod: Ka({
            values: cp,
            defaultWidth: "wide",
            formattingValues: fp,
            defaultFormattingWidth: "wide"
        })
    };
    function $a(l) {
        return (i, u = {})=>{
            const r = u.width, c = r && l.matchPatterns[r] || l.matchPatterns[l.defaultMatchWidth], d = i.match(c);
            if (!d) return null;
            const m = d[0], v = r && l.parsePatterns[r] || l.parsePatterns[l.defaultParseWidth], y = Array.isArray(v) ? mp(v, (M)=>M.test(m)) : gp(v, (M)=>M.test(m));
            let p;
            p = l.valueCallback ? l.valueCallback(y) : y, p = u.valueCallback ? u.valueCallback(p) : p;
            const _ = i.slice(m.length);
            return {
                value: p,
                rest: _
            };
        };
    }
    function gp(l, i) {
        for(const u in l)if (Object.prototype.hasOwnProperty.call(l, u) && i(l[u])) return u;
    }
    function mp(l, i) {
        for(let u = 0; u < l.length; u++)if (i(l[u])) return u;
    }
    function yp(l) {
        return (i, u = {})=>{
            const r = i.match(l.matchPattern);
            if (!r) return null;
            const c = r[0], d = i.match(l.parsePattern);
            if (!d) return null;
            let m = l.valueCallback ? l.valueCallback(d[0]) : d[0];
            m = u.valueCallback ? u.valueCallback(m) : m;
            const v = i.slice(c.length);
            return {
                value: m,
                rest: v
            };
        };
    }
    const vp = /^(\d+)(th|st|nd|rd)?/i, pp = /\d+/i, Sp = {
        narrow: /^(b|a)/i,
        abbreviated: /^(b\.?\s?c\.?|b\.?\s?c\.?\s?e\.?|a\.?\s?d\.?|c\.?\s?e\.?)/i,
        wide: /^(before christ|before common era|anno domini|common era)/i
    }, bp = {
        any: [
            /^b/i,
            /^(a|c)/i
        ]
    }, xp = {
        narrow: /^[1234]/i,
        abbreviated: /^q[1234]/i,
        wide: /^[1234](th|st|nd|rd)? quarter/i
    }, _p = {
        any: [
            /1/i,
            /2/i,
            /3/i,
            /4/i
        ]
    }, Cp = {
        narrow: /^[jfmasond]/i,
        abbreviated: /^(jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)/i,
        wide: /^(january|february|march|april|may|june|july|august|september|october|november|december)/i
    }, Rp = {
        narrow: [
            /^j/i,
            /^f/i,
            /^m/i,
            /^a/i,
            /^m/i,
            /^j/i,
            /^j/i,
            /^a/i,
            /^s/i,
            /^o/i,
            /^n/i,
            /^d/i
        ],
        any: [
            /^ja/i,
            /^f/i,
            /^mar/i,
            /^ap/i,
            /^may/i,
            /^jun/i,
            /^jul/i,
            /^au/i,
            /^s/i,
            /^o/i,
            /^n/i,
            /^d/i
        ]
    }, wp = {
        narrow: /^[smtwf]/i,
        short: /^(su|mo|tu|we|th|fr|sa)/i,
        abbreviated: /^(sun|mon|tue|wed|thu|fri|sat)/i,
        wide: /^(sunday|monday|tuesday|wednesday|thursday|friday|saturday)/i
    }, Ep = {
        narrow: [
            /^s/i,
            /^m/i,
            /^t/i,
            /^w/i,
            /^t/i,
            /^f/i,
            /^s/i
        ],
        any: [
            /^su/i,
            /^m/i,
            /^tu/i,
            /^w/i,
            /^th/i,
            /^f/i,
            /^sa/i
        ]
    }, Mp = {
        narrow: /^(a|p|mi|n|(in the|at) (morning|afternoon|evening|night))/i,
        any: /^([ap]\.?\s?m\.?|midnight|noon|(in the|at) (morning|afternoon|evening|night))/i
    }, jp = {
        any: {
            am: /^a/i,
            pm: /^p/i,
            midnight: /^mi/i,
            noon: /^no/i,
            morning: /morning/i,
            afternoon: /afternoon/i,
            evening: /evening/i,
            night: /night/i
        }
    }, Op = {
        ordinalNumber: yp({
            matchPattern: vp,
            parsePattern: pp,
            valueCallback: (l)=>parseInt(l, 10)
        }),
        era: $a({
            matchPatterns: Sp,
            defaultMatchWidth: "wide",
            parsePatterns: bp,
            defaultParseWidth: "any"
        }),
        quarter: $a({
            matchPatterns: xp,
            defaultMatchWidth: "wide",
            parsePatterns: _p,
            defaultParseWidth: "any",
            valueCallback: (l)=>l + 1
        }),
        month: $a({
            matchPatterns: Cp,
            defaultMatchWidth: "wide",
            parsePatterns: Rp,
            defaultParseWidth: "any"
        }),
        day: $a({
            matchPatterns: wp,
            defaultMatchWidth: "wide",
            parsePatterns: Ep,
            defaultParseWidth: "any"
        }),
        dayPeriod: $a({
            matchPatterns: Mp,
            defaultMatchWidth: "any",
            parsePatterns: jp,
            defaultParseWidth: "any"
        })
    }, Tp = {
        code: "en-US",
        formatDistance: Iv,
        formatLong: lp,
        formatRelative: ip,
        localize: hp,
        match: Op,
        options: {
            weekStartsOn: 0,
            firstWeekContainsDate: 1
        }
    };
    function Np(l, i) {
        const u = Ut(l, i?.in);
        return Kv(u, Wv(u)) + 1;
    }
    function Ap(l, i) {
        const u = Ut(l, i?.in), r = +Ru(u) - +$v(u);
        return Math.round(r / Ng) + 1;
    }
    function zg(l, i) {
        const u = Ut(l, i?.in), r = u.getFullYear(), c = Eu(), d = i?.firstWeekContainsDate ?? i?.locale?.options?.firstWeekContainsDate ?? c.firstWeekContainsDate ?? c.locale?.options?.firstWeekContainsDate ?? 1, m = Vn(i?.in || l, 0);
        m.setFullYear(r + 1, 0, d), m.setHours(0, 0, 0, 0);
        const v = Ia(m, i), y = Vn(i?.in || l, 0);
        y.setFullYear(r, 0, d), y.setHours(0, 0, 0, 0);
        const p = Ia(y, i);
        return +u >= +v ? r + 1 : +u >= +p ? r : r - 1;
    }
    function zp(l, i) {
        const u = Eu(), r = i?.firstWeekContainsDate ?? i?.locale?.options?.firstWeekContainsDate ?? u.firstWeekContainsDate ?? u.locale?.options?.firstWeekContainsDate ?? 1, c = zg(l, i), d = Vn(i?.in || l, 0);
        return d.setFullYear(c, 0, r), d.setHours(0, 0, 0, 0), Ia(d, i);
    }
    function Dp(l, i) {
        const u = Ut(l, i?.in), r = +Ia(u, i) - +zp(u, i);
        return Math.round(r / Ng) + 1;
    }
    function be(l, i) {
        const u = l < 0 ? "-" : "", r = Math.abs(l).toString().padStart(i, "0");
        return u + r;
    }
    const Dn = {
        y (l, i) {
            const u = l.getFullYear(), r = u > 0 ? u : 1 - u;
            return be(i === "yy" ? r % 100 : r, i.length);
        },
        M (l, i) {
            const u = l.getMonth();
            return i === "M" ? String(u + 1) : be(u + 1, 2);
        },
        d (l, i) {
            return be(l.getDate(), i.length);
        },
        a (l, i) {
            const u = l.getHours() / 12 >= 1 ? "pm" : "am";
            switch(i){
                case "a":
                case "aa":
                    return u.toUpperCase();
                case "aaa":
                    return u;
                case "aaaaa":
                    return u[0];
                default:
                    return u === "am" ? "a.m." : "p.m.";
            }
        },
        h (l, i) {
            return be(l.getHours() % 12 || 12, i.length);
        },
        H (l, i) {
            return be(l.getHours(), i.length);
        },
        m (l, i) {
            return be(l.getMinutes(), i.length);
        },
        s (l, i) {
            return be(l.getSeconds(), i.length);
        },
        S (l, i) {
            const u = i.length, r = l.getMilliseconds(), c = Math.trunc(r * Math.pow(10, u - 3));
            return be(c, i.length);
        }
    }, $l = {
        midnight: "midnight",
        noon: "noon",
        morning: "morning",
        afternoon: "afternoon",
        evening: "evening",
        night: "night"
    }, ng = {
        G: function(l, i, u) {
            const r = l.getFullYear() > 0 ? 1 : 0;
            switch(i){
                case "G":
                case "GG":
                case "GGG":
                    return u.era(r, {
                        width: "abbreviated"
                    });
                case "GGGGG":
                    return u.era(r, {
                        width: "narrow"
                    });
                default:
                    return u.era(r, {
                        width: "wide"
                    });
            }
        },
        y: function(l, i, u) {
            if (i === "yo") {
                const r = l.getFullYear(), c = r > 0 ? r : 1 - r;
                return u.ordinalNumber(c, {
                    unit: "year"
                });
            }
            return Dn.y(l, i);
        },
        Y: function(l, i, u, r) {
            const c = zg(l, r), d = c > 0 ? c : 1 - c;
            if (i === "YY") {
                const m = d % 100;
                return be(m, 2);
            }
            return i === "Yo" ? u.ordinalNumber(d, {
                unit: "year"
            }) : be(d, i.length);
        },
        R: function(l, i) {
            const u = Ag(l);
            return be(u, i.length);
        },
        u: function(l, i) {
            const u = l.getFullYear();
            return be(u, i.length);
        },
        Q: function(l, i, u) {
            const r = Math.ceil((l.getMonth() + 1) / 3);
            switch(i){
                case "Q":
                    return String(r);
                case "QQ":
                    return be(r, 2);
                case "Qo":
                    return u.ordinalNumber(r, {
                        unit: "quarter"
                    });
                case "QQQ":
                    return u.quarter(r, {
                        width: "abbreviated",
                        context: "formatting"
                    });
                case "QQQQQ":
                    return u.quarter(r, {
                        width: "narrow",
                        context: "formatting"
                    });
                default:
                    return u.quarter(r, {
                        width: "wide",
                        context: "formatting"
                    });
            }
        },
        q: function(l, i, u) {
            const r = Math.ceil((l.getMonth() + 1) / 3);
            switch(i){
                case "q":
                    return String(r);
                case "qq":
                    return be(r, 2);
                case "qo":
                    return u.ordinalNumber(r, {
                        unit: "quarter"
                    });
                case "qqq":
                    return u.quarter(r, {
                        width: "abbreviated",
                        context: "standalone"
                    });
                case "qqqqq":
                    return u.quarter(r, {
                        width: "narrow",
                        context: "standalone"
                    });
                default:
                    return u.quarter(r, {
                        width: "wide",
                        context: "standalone"
                    });
            }
        },
        M: function(l, i, u) {
            const r = l.getMonth();
            switch(i){
                case "M":
                case "MM":
                    return Dn.M(l, i);
                case "Mo":
                    return u.ordinalNumber(r + 1, {
                        unit: "month"
                    });
                case "MMM":
                    return u.month(r, {
                        width: "abbreviated",
                        context: "formatting"
                    });
                case "MMMMM":
                    return u.month(r, {
                        width: "narrow",
                        context: "formatting"
                    });
                default:
                    return u.month(r, {
                        width: "wide",
                        context: "formatting"
                    });
            }
        },
        L: function(l, i, u) {
            const r = l.getMonth();
            switch(i){
                case "L":
                    return String(r + 1);
                case "LL":
                    return be(r + 1, 2);
                case "Lo":
                    return u.ordinalNumber(r + 1, {
                        unit: "month"
                    });
                case "LLL":
                    return u.month(r, {
                        width: "abbreviated",
                        context: "standalone"
                    });
                case "LLLLL":
                    return u.month(r, {
                        width: "narrow",
                        context: "standalone"
                    });
                default:
                    return u.month(r, {
                        width: "wide",
                        context: "standalone"
                    });
            }
        },
        w: function(l, i, u, r) {
            const c = Dp(l, r);
            return i === "wo" ? u.ordinalNumber(c, {
                unit: "week"
            }) : be(c, i.length);
        },
        I: function(l, i, u) {
            const r = Ap(l);
            return i === "Io" ? u.ordinalNumber(r, {
                unit: "week"
            }) : be(r, i.length);
        },
        d: function(l, i, u) {
            return i === "do" ? u.ordinalNumber(l.getDate(), {
                unit: "date"
            }) : Dn.d(l, i);
        },
        D: function(l, i, u) {
            const r = Np(l);
            return i === "Do" ? u.ordinalNumber(r, {
                unit: "dayOfYear"
            }) : be(r, i.length);
        },
        E: function(l, i, u) {
            const r = l.getDay();
            switch(i){
                case "E":
                case "EE":
                case "EEE":
                    return u.day(r, {
                        width: "abbreviated",
                        context: "formatting"
                    });
                case "EEEEE":
                    return u.day(r, {
                        width: "narrow",
                        context: "formatting"
                    });
                case "EEEEEE":
                    return u.day(r, {
                        width: "short",
                        context: "formatting"
                    });
                default:
                    return u.day(r, {
                        width: "wide",
                        context: "formatting"
                    });
            }
        },
        e: function(l, i, u, r) {
            const c = l.getDay(), d = (c - r.weekStartsOn + 8) % 7 || 7;
            switch(i){
                case "e":
                    return String(d);
                case "ee":
                    return be(d, 2);
                case "eo":
                    return u.ordinalNumber(d, {
                        unit: "day"
                    });
                case "eee":
                    return u.day(c, {
                        width: "abbreviated",
                        context: "formatting"
                    });
                case "eeeee":
                    return u.day(c, {
                        width: "narrow",
                        context: "formatting"
                    });
                case "eeeeee":
                    return u.day(c, {
                        width: "short",
                        context: "formatting"
                    });
                default:
                    return u.day(c, {
                        width: "wide",
                        context: "formatting"
                    });
            }
        },
        c: function(l, i, u, r) {
            const c = l.getDay(), d = (c - r.weekStartsOn + 8) % 7 || 7;
            switch(i){
                case "c":
                    return String(d);
                case "cc":
                    return be(d, i.length);
                case "co":
                    return u.ordinalNumber(d, {
                        unit: "day"
                    });
                case "ccc":
                    return u.day(c, {
                        width: "abbreviated",
                        context: "standalone"
                    });
                case "ccccc":
                    return u.day(c, {
                        width: "narrow",
                        context: "standalone"
                    });
                case "cccccc":
                    return u.day(c, {
                        width: "short",
                        context: "standalone"
                    });
                default:
                    return u.day(c, {
                        width: "wide",
                        context: "standalone"
                    });
            }
        },
        i: function(l, i, u) {
            const r = l.getDay(), c = r === 0 ? 7 : r;
            switch(i){
                case "i":
                    return String(c);
                case "ii":
                    return be(c, i.length);
                case "io":
                    return u.ordinalNumber(c, {
                        unit: "day"
                    });
                case "iii":
                    return u.day(r, {
                        width: "abbreviated",
                        context: "formatting"
                    });
                case "iiiii":
                    return u.day(r, {
                        width: "narrow",
                        context: "formatting"
                    });
                case "iiiiii":
                    return u.day(r, {
                        width: "short",
                        context: "formatting"
                    });
                default:
                    return u.day(r, {
                        width: "wide",
                        context: "formatting"
                    });
            }
        },
        a: function(l, i, u) {
            const c = l.getHours() / 12 >= 1 ? "pm" : "am";
            switch(i){
                case "a":
                case "aa":
                    return u.dayPeriod(c, {
                        width: "abbreviated",
                        context: "formatting"
                    });
                case "aaa":
                    return u.dayPeriod(c, {
                        width: "abbreviated",
                        context: "formatting"
                    }).toLowerCase();
                case "aaaaa":
                    return u.dayPeriod(c, {
                        width: "narrow",
                        context: "formatting"
                    });
                default:
                    return u.dayPeriod(c, {
                        width: "wide",
                        context: "formatting"
                    });
            }
        },
        b: function(l, i, u) {
            const r = l.getHours();
            let c;
            switch(r === 12 ? c = $l.noon : r === 0 ? c = $l.midnight : c = r / 12 >= 1 ? "pm" : "am", i){
                case "b":
                case "bb":
                    return u.dayPeriod(c, {
                        width: "abbreviated",
                        context: "formatting"
                    });
                case "bbb":
                    return u.dayPeriod(c, {
                        width: "abbreviated",
                        context: "formatting"
                    }).toLowerCase();
                case "bbbbb":
                    return u.dayPeriod(c, {
                        width: "narrow",
                        context: "formatting"
                    });
                default:
                    return u.dayPeriod(c, {
                        width: "wide",
                        context: "formatting"
                    });
            }
        },
        B: function(l, i, u) {
            const r = l.getHours();
            let c;
            switch(r >= 17 ? c = $l.evening : r >= 12 ? c = $l.afternoon : r >= 4 ? c = $l.morning : c = $l.night, i){
                case "B":
                case "BB":
                case "BBB":
                    return u.dayPeriod(c, {
                        width: "abbreviated",
                        context: "formatting"
                    });
                case "BBBBB":
                    return u.dayPeriod(c, {
                        width: "narrow",
                        context: "formatting"
                    });
                default:
                    return u.dayPeriod(c, {
                        width: "wide",
                        context: "formatting"
                    });
            }
        },
        h: function(l, i, u) {
            if (i === "ho") {
                let r = l.getHours() % 12;
                return r === 0 && (r = 12), u.ordinalNumber(r, {
                    unit: "hour"
                });
            }
            return Dn.h(l, i);
        },
        H: function(l, i, u) {
            return i === "Ho" ? u.ordinalNumber(l.getHours(), {
                unit: "hour"
            }) : Dn.H(l, i);
        },
        K: function(l, i, u) {
            const r = l.getHours() % 12;
            return i === "Ko" ? u.ordinalNumber(r, {
                unit: "hour"
            }) : be(r, i.length);
        },
        k: function(l, i, u) {
            let r = l.getHours();
            return r === 0 && (r = 24), i === "ko" ? u.ordinalNumber(r, {
                unit: "hour"
            }) : be(r, i.length);
        },
        m: function(l, i, u) {
            return i === "mo" ? u.ordinalNumber(l.getMinutes(), {
                unit: "minute"
            }) : Dn.m(l, i);
        },
        s: function(l, i, u) {
            return i === "so" ? u.ordinalNumber(l.getSeconds(), {
                unit: "second"
            }) : Dn.s(l, i);
        },
        S: function(l, i) {
            return Dn.S(l, i);
        },
        X: function(l, i, u) {
            const r = l.getTimezoneOffset();
            if (r === 0) return "Z";
            switch(i){
                case "X":
                    return ag(r);
                case "XXXX":
                case "XX":
                    return il(r);
                default:
                    return il(r, ":");
            }
        },
        x: function(l, i, u) {
            const r = l.getTimezoneOffset();
            switch(i){
                case "x":
                    return ag(r);
                case "xxxx":
                case "xx":
                    return il(r);
                default:
                    return il(r, ":");
            }
        },
        O: function(l, i, u) {
            const r = l.getTimezoneOffset();
            switch(i){
                case "O":
                case "OO":
                case "OOO":
                    return "GMT" + lg(r, ":");
                default:
                    return "GMT" + il(r, ":");
            }
        },
        z: function(l, i, u) {
            const r = l.getTimezoneOffset();
            switch(i){
                case "z":
                case "zz":
                case "zzz":
                    return "GMT" + lg(r, ":");
                default:
                    return "GMT" + il(r, ":");
            }
        },
        t: function(l, i, u) {
            const r = Math.trunc(+l / 1e3);
            return be(r, i.length);
        },
        T: function(l, i, u) {
            return be(+l, i.length);
        }
    };
    function lg(l, i = "") {
        const u = l > 0 ? "-" : "+", r = Math.abs(l), c = Math.trunc(r / 60), d = r % 60;
        return d === 0 ? u + String(c) : u + String(c) + i + be(d, 2);
    }
    function ag(l, i) {
        return l % 60 === 0 ? (l > 0 ? "-" : "+") + be(Math.abs(l) / 60, 2) : il(l, i);
    }
    function il(l, i = "") {
        const u = l > 0 ? "-" : "+", r = Math.abs(l), c = be(Math.trunc(r / 60), 2), d = be(r % 60, 2);
        return u + c + i + d;
    }
    const ig = (l, i)=>{
        switch(l){
            case "P":
                return i.date({
                    width: "short"
                });
            case "PP":
                return i.date({
                    width: "medium"
                });
            case "PPP":
                return i.date({
                    width: "long"
                });
            default:
                return i.date({
                    width: "full"
                });
        }
    }, Dg = (l, i)=>{
        switch(l){
            case "p":
                return i.time({
                    width: "short"
                });
            case "pp":
                return i.time({
                    width: "medium"
                });
            case "ppp":
                return i.time({
                    width: "long"
                });
            default:
                return i.time({
                    width: "full"
                });
        }
    }, Hp = (l, i)=>{
        const u = l.match(/(P+)(p+)?/) || [], r = u[1], c = u[2];
        if (!c) return ig(l, i);
        let d;
        switch(r){
            case "P":
                d = i.dateTime({
                    width: "short"
                });
                break;
            case "PP":
                d = i.dateTime({
                    width: "medium"
                });
                break;
            case "PPP":
                d = i.dateTime({
                    width: "long"
                });
                break;
            default:
                d = i.dateTime({
                    width: "full"
                });
                break;
        }
        return d.replace("{{date}}", ig(r, i)).replace("{{time}}", Dg(c, i));
    }, qp = {
        p: Dg,
        P: Hp
    }, Up = /^D+$/, Vp = /^Y+$/, Lp = [
        "D",
        "DD",
        "YY",
        "YYYY"
    ];
    function Gp(l) {
        return Up.test(l);
    }
    function Bp(l) {
        return Vp.test(l);
    }
    function Qp(l, i, u) {
        const r = Yp(l, i, u);
        if (console.warn(r), Lp.includes(l)) throw new RangeError(r);
    }
    function Yp(l, i, u) {
        const r = l[0] === "Y" ? "years" : "days of the month";
        return `Use \`${l.toLowerCase()}\` instead of \`${l}\` (in \`${i}\`) for formatting ${r} to the input \`${u}\`; see: https://github.com/date-fns/date-fns/blob/master/docs/unicodeTokens.md`;
    }
    const Xp = /[yYQqMLwIdDecihHKkms]o|(\w)\1*|''|'(''|[^'])+('|$)|./g, Fp = /P+p+|P+|p+|''|'(''|[^'])+('|$)|./g, Zp = /^'([^]*?)'?$/, Kp = /''/g, $p = /[a-zA-Z]/;
    function Jp(l, i, u) {
        const r = Eu(), c = r.locale ?? Tp, d = r.firstWeekContainsDate ?? r.locale?.options?.firstWeekContainsDate ?? 1, m = r.weekStartsOn ?? r.locale?.options?.weekStartsOn ?? 0, v = Ut(l, u?.in);
        if (!kv(v)) throw new RangeError("Invalid time value");
        let y = i.match(Fp).map((_)=>{
            const M = _[0];
            if (M === "p" || M === "P") {
                const j = qp[M];
                return j(_, c.formatLong);
            }
            return _;
        }).join("").match(Xp).map((_)=>{
            if (_ === "''") return {
                isToken: !1,
                value: "'"
            };
            const M = _[0];
            if (M === "'") return {
                isToken: !1,
                value: kp(_)
            };
            if (ng[M]) return {
                isToken: !0,
                value: _
            };
            if (M.match($p)) throw new RangeError("Format string contains an unescaped latin alphabet character `" + M + "`");
            return {
                isToken: !1,
                value: _
            };
        });
        c.localize.preprocessor && (y = c.localize.preprocessor(v, y));
        const p = {
            firstWeekContainsDate: d,
            weekStartsOn: m,
            locale: c
        };
        return y.map((_)=>{
            if (!_.isToken) return _.value;
            const M = _.value;
            (Bp(M) || Gp(M)) && Qp(M, i, String(l));
            const j = ng[M[0]];
            return j(v, M, c.localize, p);
        }).join("");
    }
    function kp(l) {
        const i = l.match(Zp);
        return i ? i[1].replace(Kp, "'") : l;
    }
    const Wp = {
        debug: "text-gray-500",
        info: "text-blue-600",
        notice: "text-cyan-600",
        warning: "text-yellow-600",
        error: "text-red-600",
        critical: "text-red-800 font-bold"
    }, Pp = (l)=>[
            {
                id: "id",
                accessorKey: "id",
                header: "ID",
                cell: (i)=>h.jsx("div", {
                        className: "text-sm text-gray-600 font-mono",
                        children: i.getValue()
                    }),
                size: 45
            },
            {
                id: "timestamp",
                accessorKey: "timestamp",
                header: "Timestamp",
                cell: (i)=>{
                    const u = i.getValue(), r = new Date(u);
                    return h.jsx("div", {
                        className: "text-sm text-gray-900 whitespace-nowrap font-mono",
                        children: Jp(r, "MMM d, yyyy HH:mm:ss.SSS")
                    });
                },
                size: 200
            },
            {
                id: "severity",
                accessorKey: "severity",
                header: "Severity",
                cell: (i)=>{
                    const u = i.getValue(), r = Wp[u] || "text-gray-900";
                    return h.jsx("div", {
                        className: `text-sm font-medium ${r}`,
                        children: u
                    });
                },
                size: 75
            },
            {
                id: "node",
                accessorKey: "node",
                header: "Node",
                cell: (i)=>h.jsx("div", {
                        className: "text-sm text-gray-700 font-mono truncate",
                        children: i.getValue()
                    }),
                size: 120
            },
            {
                id: "erlang_pid",
                accessorKey: "erlang_pid",
                header: "PID",
                cell: (i)=>{
                    const u = i.getValue();
                    return h.jsxs("div", {
                        className: "flex items-center gap-2",
                        children: [
                            h.jsx("span", {
                                className: "text-sm text-gray-600 font-mono",
                                children: u
                            }),
                            l && h.jsx("button", {
                                onClick: ()=>l(u),
                                className: "text-xs text-blue-600 hover:text-blue-800 hover:underline",
                                children: "Filter"
                            })
                        ]
                    });
                },
                size: 70
            },
            {
                id: "subsystem",
                accessorKey: "subsystem",
                header: "Subsystem",
                cell: (i)=>{
                    const u = i.getValue();
                    return h.jsx("div", {
                        className: "text-sm text-gray-700",
                        children: u || "-"
                    });
                },
                size: 110
            },
            {
                id: "message",
                accessorKey: "message",
                header: "Message",
                cell: (i)=>{
                    const u = i.getValue();
                    return h.jsx("div", {
                        className: "text-sm text-gray-900 whitespace-pre-wrap font-mono max-w-5xl",
                        children: u
                    });
                },
                size: 800
            },
            {
                id: "labels",
                accessorKey: "labels",
                header: "Labels",
                cell: (i)=>{
                    const u = i.getValue(), r = Object.entries(u).filter(([c, d])=>d).map(([c])=>c).sort();
                    return h.jsx("div", {
                        className: "flex flex-wrap gap-1",
                        children: r.map((c)=>h.jsx("span", {
                                className: "inline-block px-2 py-1 text-xs bg-blue-100 text-blue-800 rounded",
                                children: c
                            }, c))
                    });
                },
                size: 180
            },
            {
                id: "doc_url",
                accessorKey: "doc_url",
                header: "Doc URL",
                cell: (i)=>{
                    const u = i.getValue();
                    return u ? h.jsx("a", {
                        href: u,
                        target: "_blank",
                        rel: "noopener noreferrer",
                        className: "text-sm text-blue-600 hover:text-blue-800 underline",
                        children: "View"
                    }) : h.jsx("span", {
                        className: "text-sm text-gray-400",
                        children: "-"
                    });
                },
                size: 75
            },
            {
                id: "resolution_or_discussion_url",
                accessorKey: "resolution_or_discussion_url",
                header: "Resolution",
                cell: (i)=>{
                    const u = i.getValue();
                    return u ? h.jsx("a", {
                        href: u,
                        target: "_blank",
                        rel: "noopener noreferrer",
                        className: "text-sm text-blue-600 hover:text-blue-800 underline",
                        children: "View"
                    }) : h.jsx("span", {
                        className: "text-sm text-gray-400",
                        children: "-"
                    });
                },
                size: 85
            }
        ], ug = {
        doc_url: !1
    };
    function io({ data: l, onPidFilterClick: i }) {
        const [u, r] = ie.useState(ug), [c, d] = ie.useState(!1), m = Pp(i), v = Yv({
            data: l,
            columns: m,
            state: {
                columnVisibility: u
            },
            onColumnVisibilityChange: r,
            getCoreRowModel: Lv()
        }), y = ()=>{
            r(ug);
        };
        return h.jsxs("div", {
            className: "bg-white shadow-sm border border-gray-200 rounded-lg overflow-hidden",
            children: [
                h.jsxs("div", {
                    className: "p-4 border-b border-gray-200 flex items-center justify-between bg-gray-50",
                    children: [
                        h.jsxs("div", {
                            className: "flex items-center gap-3",
                            children: [
                                h.jsxs("button", {
                                    onClick: ()=>d(!c),
                                    className: "flex items-center gap-2 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded font-medium text-sm transition-colors",
                                    children: [
                                        h.jsx("svg", {
                                            className: "w-4 h-4",
                                            fill: "none",
                                            stroke: "currentColor",
                                            viewBox: "0 0 24 24",
                                            children: h.jsx("path", {
                                                strokeLinecap: "round",
                                                strokeLinejoin: "round",
                                                strokeWidth: 2,
                                                d: "M9 17V7m0 10a2 2 0 01-2 2H5a2 2 0 01-2-2V7a2 2 0 012-2h2a2 2 0 012 2m0 10a2 2 0 002 2h2a2 2 0 002-2M9 7a2 2 0 012-2h2a2 2 0 012 2m0 0h6m-6 0v10"
                                            })
                                        }),
                                        c ? "Done" : "Select Columns"
                                    ]
                                }),
                                !c && h.jsxs("span", {
                                    className: "text-xs text-gray-500",
                                    children: [
                                        v.getVisibleLeafColumns().length,
                                        " of ",
                                        v.getAllLeafColumns().length,
                                        " columns visible"
                                    ]
                                })
                            ]
                        }),
                        c && h.jsxs("div", {
                            className: "flex items-center gap-2",
                            children: [
                                h.jsx("button", {
                                    onClick: ()=>v.toggleAllColumnsVisible(!0),
                                    className: "text-sm px-3 py-1.5 bg-white border border-gray-300 hover:bg-gray-50 rounded font-medium",
                                    children: "Show All"
                                }),
                                h.jsx("button", {
                                    onClick: y,
                                    className: "text-sm px-3 py-1.5 bg-white border border-gray-300 hover:bg-gray-50 rounded font-medium",
                                    children: "Reset Columns"
                                })
                            ]
                        })
                    ]
                }),
                c && h.jsx("div", {
                    className: "p-4 border-b border-gray-200 bg-gray-50",
                    children: h.jsx("div", {
                        className: "grid grid-cols-2 sm:grid-cols-3 md:grid-cols-5 gap-3",
                        children: v.getAllLeafColumns().map((p)=>h.jsxs("label", {
                                className: "flex items-center space-x-2 text-sm",
                                children: [
                                    h.jsx("input", {
                                        type: "checkbox",
                                        checked: p.getIsVisible(),
                                        onChange: p.getToggleVisibilityHandler(),
                                        className: "rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                                    }),
                                    h.jsx("span", {
                                        className: "text-gray-700",
                                        children: p.id
                                    })
                                ]
                            }, p.id))
                    })
                }),
                h.jsx("div", {
                    className: "overflow-x-auto",
                    children: h.jsxs("table", {
                        className: "min-w-full divide-y divide-gray-200",
                        children: [
                            h.jsx("thead", {
                                className: "bg-gray-50",
                                children: v.getHeaderGroups().map((p)=>h.jsx("tr", {
                                        children: p.headers.map((_)=>h.jsx("th", {
                                                className: "px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                                style: {
                                                    width: _.getSize()
                                                },
                                                children: _.isPlaceholder ? null : Ph(_.column.columnDef.header, _.getContext())
                                            }, _.id))
                                    }, p.id))
                            }),
                            h.jsx("tbody", {
                                className: "bg-white divide-y divide-gray-200",
                                children: v.getRowModel().rows.map((p)=>h.jsx("tr", {
                                        className: "hover:bg-gray-50",
                                        children: p.getVisibleCells().map((_)=>h.jsx("td", {
                                                className: "px-4 py-3",
                                                children: Ph(_.column.columnDef.cell, _.getContext())
                                            }, _.id))
                                    }, p.id))
                            })
                        ]
                    })
                }),
                l.length === 0 && h.jsx("div", {
                    className: "text-center py-12 text-gray-500",
                    children: "No log entries found. Try adjusting your filters."
                })
            ]
        });
    }
    function Jl(l) {
        return (typeof l == "string" ? new Date(l) : l).toLocaleDateString("en-US", {
            year: "numeric",
            month: "short",
            day: "numeric"
        });
    }
    function Ip(l) {
        const i = l.split("/");
        return i[i.length - 1] || l;
    }
    function eS({ fileMetadata: l }) {
        if (!l || l.length === 0) return h.jsx("div", {
            className: "bg-white shadow-sm border border-gray-200 rounded-lg p-8 text-center text-gray-500",
            children: "No file metadata found in database"
        });
        const i = [
            ...l
        ].sort((u, r)=>r.total_entries - u.total_entries);
        return h.jsx("div", {
            className: "space-y-6",
            children: i.map((u, r)=>h.jsxs("div", {
                    className: "bg-white shadow-sm border border-gray-200 rounded-lg p-6",
                    children: [
                        r > 0 && h.jsx("div", {
                            className: "mb-4"
                        }),
                        h.jsx("h3", {
                            className: "text-lg font-semibold text-gray-900 mb-4",
                            children: Ip(u.file_path)
                        }),
                        h.jsxs("div", {
                            className: "grid grid-cols-1 md:grid-cols-2 gap-4",
                            children: [
                                u.rabbitmq_versions.length > 0 && h.jsxs("div", {
                                    children: [
                                        h.jsx("dt", {
                                            className: "text-sm font-medium text-gray-500",
                                            children: "RabbitMQ Version"
                                        }),
                                        h.jsx("dd", {
                                            className: "mt-1 text-sm text-gray-900",
                                            children: [
                                                ...new Set(u.rabbitmq_versions)
                                            ].join(", ")
                                        })
                                    ]
                                }),
                                u.erlang_versions.length > 0 && h.jsxs("div", {
                                    children: [
                                        h.jsx("dt", {
                                            className: "text-sm font-medium text-gray-500",
                                            children: "Erlang Version"
                                        }),
                                        h.jsx("dd", {
                                            className: "mt-1 text-sm text-gray-900",
                                            children: [
                                                ...new Set(u.erlang_versions)
                                            ].join(", ")
                                        })
                                    ]
                                }),
                                u.tls_library && h.jsxs("div", {
                                    children: [
                                        h.jsx("dt", {
                                            className: "text-sm font-medium text-gray-500",
                                            children: "TLS Library"
                                        }),
                                        h.jsx("dd", {
                                            className: "mt-1 text-sm text-gray-900",
                                            children: u.tls_library
                                        })
                                    ]
                                }),
                                u.oldest_entry_at && h.jsxs("div", {
                                    children: [
                                        h.jsx("dt", {
                                            className: "text-sm font-medium text-gray-500",
                                            children: "Oldest Entry"
                                        }),
                                        h.jsx("dd", {
                                            className: "mt-1 text-sm text-gray-900",
                                            children: Jl(u.oldest_entry_at)
                                        })
                                    ]
                                }),
                                u.most_recent_entry_at && h.jsxs("div", {
                                    children: [
                                        h.jsx("dt", {
                                            className: "text-sm font-medium text-gray-500",
                                            children: "Most Recent Entry"
                                        }),
                                        h.jsx("dd", {
                                            className: "mt-1 text-sm text-gray-900",
                                            children: Jl(u.most_recent_entry_at)
                                        })
                                    ]
                                }),
                                h.jsxs("div", {
                                    children: [
                                        h.jsx("dt", {
                                            className: "text-sm font-medium text-gray-500",
                                            children: "Total Lines"
                                        }),
                                        h.jsx("dd", {
                                            className: "mt-1 text-sm text-gray-900",
                                            children: u.total_lines.toLocaleString()
                                        })
                                    ]
                                }),
                                h.jsxs("div", {
                                    children: [
                                        h.jsx("dt", {
                                            className: "text-sm font-medium text-gray-500",
                                            children: "Total Entries"
                                        }),
                                        h.jsx("dd", {
                                            className: "mt-1 text-sm text-gray-900",
                                            children: u.total_entries.toLocaleString()
                                        })
                                    ]
                                })
                            ]
                        }),
                        u.nodes.length > 0 && h.jsxs("div", {
                            className: "mt-4",
                            children: [
                                h.jsx("dt", {
                                    className: "text-sm font-medium text-gray-500",
                                    children: "Nodes"
                                }),
                                h.jsx("dd", {
                                    className: "mt-2 flex flex-wrap gap-2",
                                    children: u.nodes.map((c)=>h.jsx("span", {
                                            className: "inline-flex items-center px-2.5 py-0.5 rounded-md text-sm font-medium bg-blue-100 text-blue-800",
                                            children: c
                                        }, c))
                                })
                            ]
                        }),
                        u.subsystems.length > 0 && h.jsxs("div", {
                            className: "mt-4",
                            children: [
                                h.jsx("dt", {
                                    className: "text-sm font-medium text-gray-500",
                                    children: "Subsystems"
                                }),
                                h.jsx("dd", {
                                    className: "mt-2 flex flex-wrap gap-2",
                                    children: u.subsystems.map((c)=>h.jsx("span", {
                                            className: "inline-flex items-center px-2.5 py-0.5 rounded-md text-sm font-medium bg-green-100 text-green-800",
                                            children: c
                                        }, c))
                                })
                            ]
                        }),
                        u.labels.length > 0 && h.jsxs("div", {
                            className: "mt-4",
                            children: [
                                h.jsx("dt", {
                                    className: "text-sm font-medium text-gray-500",
                                    children: "Labels"
                                }),
                                h.jsx("dd", {
                                    className: "mt-2 flex flex-wrap gap-2",
                                    children: u.labels.map((c)=>h.jsx("span", {
                                            className: "inline-flex items-center px-2.5 py-0.5 rounded-md text-sm font-medium bg-purple-100 text-purple-800",
                                            children: c
                                        }, c))
                                })
                            ]
                        }),
                        u.enabled_plugins.length > 0 && h.jsxs("div", {
                            className: "mt-4",
                            children: [
                                h.jsxs("dt", {
                                    className: "text-sm font-medium text-gray-500",
                                    children: [
                                        "Enabled Plugins (",
                                        u.enabled_plugins.length,
                                        ")"
                                    ]
                                }),
                                h.jsx("dd", {
                                    className: "mt-2 flex flex-wrap gap-2",
                                    children: u.enabled_plugins.map((c)=>h.jsx("span", {
                                            className: "inline-flex items-center px-2.5 py-0.5 rounded-md text-sm font-medium bg-orange-100 text-orange-800",
                                            children: c
                                        }, c))
                                })
                            ]
                        })
                    ]
                }, u.file_path))
        });
    }
    function tS({ fileMetadata: l, onViewFullMetadata: i }) {
        const [u, r] = ie.useState(!0);
        if (!l || l.length === 0) return null;
        const c = new Set(l.flatMap((y)=>y.rabbitmq_versions)), d = Array.from(new Set(l.flatMap((y)=>y.enabled_plugins))).sort(), m = l.map((y)=>y.oldest_entry_at).filter((y)=>y !== null).sort()[0], v = l.map((y)=>y.most_recent_entry_at).filter((y)=>y !== null).sort().reverse()[0];
        return h.jsxs("div", {
            className: "bg-white shadow-sm border border-gray-200 rounded-lg mb-4",
            children: [
                h.jsxs("button", {
                    onClick: ()=>r(!u),
                    className: "w-full px-4 py-3 flex items-center justify-between text-left hover:bg-gray-50 transition-colors",
                    children: [
                        h.jsx("h3", {
                            className: "text-sm font-semibold text-gray-900",
                            children: "File Set Overview"
                        }),
                        h.jsx("svg", {
                            className: `w-5 h-5 text-gray-500 transition-transform ${u ? "rotate-180" : ""}`,
                            fill: "none",
                            stroke: "currentColor",
                            viewBox: "0 0 24 24",
                            children: h.jsx("path", {
                                strokeLinecap: "round",
                                strokeLinejoin: "round",
                                strokeWidth: 2,
                                d: "M19 9l-7 7-7-7"
                            })
                        })
                    ]
                }),
                u && h.jsxs("div", {
                    className: "px-4 pb-4 space-y-3 border-t border-gray-100",
                    children: [
                        c.size > 0 && h.jsxs("div", {
                            className: "pt-3",
                            children: [
                                h.jsx("dt", {
                                    className: "text-xs font-medium text-gray-500",
                                    children: "RabbitMQ Version"
                                }),
                                h.jsx("dd", {
                                    className: "mt-1 text-sm text-gray-900",
                                    children: Array.from(c).join(", ")
                                })
                            ]
                        }),
                        m && v && h.jsxs("div", {
                            children: [
                                h.jsx("dt", {
                                    className: "text-xs font-medium text-gray-500",
                                    children: "Time Range"
                                }),
                                h.jsxs("dd", {
                                    className: "mt-1 text-sm text-gray-900",
                                    children: [
                                        Jl(m),
                                        " → ",
                                        Jl(v)
                                    ]
                                })
                            ]
                        }),
                        h.jsxs("div", {
                            children: [
                                h.jsx("dt", {
                                    className: "text-xs font-medium text-gray-500",
                                    children: "Total Entries / Lines"
                                }),
                                h.jsxs("dd", {
                                    className: "mt-1 text-sm text-gray-900",
                                    children: [
                                        l.reduce((y, p)=>y + p.total_entries, 0).toLocaleString(),
                                        " /",
                                        " ",
                                        l.reduce((y, p)=>y + p.total_lines, 0).toLocaleString()
                                    ]
                                })
                            ]
                        }),
                        d.length > 0 && h.jsxs("div", {
                            children: [
                                h.jsxs("dt", {
                                    className: "text-xs font-medium text-gray-500",
                                    children: [
                                        "Enabled Plugins (",
                                        d.length,
                                        ")"
                                    ]
                                }),
                                h.jsxs("dd", {
                                    className: "mt-2 flex flex-wrap gap-1",
                                    children: [
                                        d.slice(0, 10).map((y)=>h.jsx("span", {
                                                className: "inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-orange-100 text-orange-800",
                                                children: y
                                            }, y)),
                                        d.length > 10 && h.jsxs("span", {
                                            className: "inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-gray-100 text-gray-600",
                                            children: [
                                                "+",
                                                d.length - 10,
                                                " more"
                                            ]
                                        })
                                    ]
                                })
                            ]
                        }),
                        h.jsx("div", {
                            className: "pt-2",
                            children: h.jsx("button", {
                                onClick: i,
                                className: "text-xs text-blue-600 hover:text-blue-800 font-medium",
                                children: "View Full Metadata →"
                            })
                        })
                    ]
                })
            ]
        });
    }
    function nS({ metadata: l, filters: i, onFilterChange: u }) {
        const r = (d, m)=>{
            u({
                ...i,
                [d]: m
            });
        }, c = ()=>{
            u({
                limit: 1e3
            });
        };
        return h.jsxs("div", {
            className: "bg-white shadow-sm border border-gray-200 rounded-lg p-6 space-y-6",
            children: [
                h.jsxs("div", {
                    className: "flex items-center justify-between",
                    children: [
                        h.jsx("h2", {
                            className: "text-lg font-semibold text-gray-900",
                            children: "Filters"
                        }),
                        h.jsx("button", {
                            onClick: c,
                            className: "text-sm text-blue-600 hover:text-blue-800",
                            children: "Revert to Default View"
                        })
                    ]
                }),
                h.jsxs("div", {
                    className: "space-y-6",
                    children: [
                        h.jsxs("div", {
                            className: "bg-blue-50 border border-blue-200 rounded-md p-3",
                            children: [
                                h.jsx("h3", {
                                    className: "text-sm font-semibold text-blue-900 mb-1",
                                    children: "Preset: Errors, Exceptions"
                                }),
                                h.jsx("p", {
                                    className: "text-xs text-blue-700",
                                    children: "Shows log entries with error severity OR entries labelled as erl_process_crash or exceptions"
                                })
                            ]
                        }),
                        h.jsxs("div", {
                            className: "border-b border-gray-200 pb-4",
                            children: [
                                h.jsx("h3", {
                                    className: "text-sm font-semibold text-gray-900 mb-3",
                                    children: "Time Range"
                                }),
                                h.jsxs("div", {
                                    className: "space-y-4",
                                    children: [
                                        h.jsxs("div", {
                                            children: [
                                                h.jsx("label", {
                                                    className: "block text-sm font-medium text-gray-700 mb-1",
                                                    children: "Since Time"
                                                }),
                                                h.jsx("input", {
                                                    type: "text",
                                                    value: i.since_time || "",
                                                    onChange: (d)=>r("since_time", d.target.value),
                                                    placeholder: "2025-10-27 or '2 days ago'",
                                                    className: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                                })
                                            ]
                                        }),
                                        h.jsxs("div", {
                                            children: [
                                                h.jsx("label", {
                                                    className: "block text-sm font-medium text-gray-700 mb-1",
                                                    children: "To Time"
                                                }),
                                                h.jsx("input", {
                                                    type: "text",
                                                    value: i.to_time || "",
                                                    onChange: (d)=>r("to_time", d.target.value),
                                                    placeholder: "2025-10-27 or 'now'",
                                                    className: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                                })
                                            ]
                                        })
                                    ]
                                })
                            ]
                        }),
                        h.jsxs("div", {
                            className: "border-b border-gray-200 pb-4",
                            children: [
                                h.jsx("h3", {
                                    className: "text-sm font-semibold text-gray-900 mb-3",
                                    children: "Node Filter"
                                }),
                                h.jsxs("div", {
                                    children: [
                                        h.jsx("label", {
                                            className: "block text-sm font-medium text-gray-700 mb-1",
                                            children: "Node"
                                        }),
                                        h.jsxs("select", {
                                            value: i.node || "",
                                            onChange: (d)=>r("node", d.target.value || void 0),
                                            className: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                                            children: [
                                                h.jsx("option", {
                                                    value: "",
                                                    children: "All"
                                                }),
                                                l?.nodes.map((d)=>h.jsx("option", {
                                                        value: d,
                                                        children: d
                                                    }, d))
                                            ]
                                        })
                                    ]
                                })
                            ]
                        }),
                        h.jsxs("div", {
                            children: [
                                h.jsx("label", {
                                    className: "block text-sm font-medium text-gray-700 mb-1",
                                    children: "Log entry rows to load (maximum)"
                                }),
                                h.jsxs("select", {
                                    value: i.limit || 1e3,
                                    onChange: (d)=>r("limit", parseInt(d.target.value)),
                                    className: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                                    children: [
                                        h.jsx("option", {
                                            value: 500,
                                            children: "500"
                                        }),
                                        h.jsx("option", {
                                            value: 1e3,
                                            children: "1000"
                                        }),
                                        h.jsx("option", {
                                            value: 3e3,
                                            children: "3000"
                                        }),
                                        h.jsx("option", {
                                            value: 5e3,
                                            children: "5000"
                                        }),
                                        h.jsx("option", {
                                            value: 1e4,
                                            children: "10000"
                                        }),
                                        h.jsx("option", {
                                            value: 2e4,
                                            children: "20000"
                                        }),
                                        h.jsx("option", {
                                            value: 5e4,
                                            children: "50000"
                                        })
                                    ]
                                })
                            ]
                        })
                    ]
                })
            ]
        });
    }
    const lS = "modulepreload", aS = function(l) {
        return "/" + l;
    }, sg = {}, iS = function(i, u, r) {
        let c = Promise.resolve();
        if (u && u.length > 0) {
            let p = function(_) {
                return Promise.all(_.map((M)=>Promise.resolve(M).then((j)=>({
                            status: "fulfilled",
                            value: j
                        }), (j)=>({
                            status: "rejected",
                            reason: j
                        }))));
            };
            var m = p;
            document.getElementsByTagName("link");
            const v = document.querySelector("meta[property=csp-nonce]"), y = v?.nonce || v?.getAttribute("nonce");
            c = p(u.map((_)=>{
                if (_ = aS(_), _ in sg) return;
                sg[_] = !0;
                const M = _.endsWith(".css"), j = M ? '[rel="stylesheet"]' : "";
                if (document.querySelector(`link[href="${_}"]${j}`)) return;
                const T = document.createElement("link");
                if (T.rel = M ? "stylesheet" : lS, M || (T.as = "script"), T.crossOrigin = "", T.href = _, y && T.setAttribute("nonce", y), document.head.appendChild(T), M) return new Promise((U, q)=>{
                    T.addEventListener("load", U), T.addEventListener("error", ()=>q(new Error(`Unable to preload CSS for ${_}`)));
                });
            }));
        }
        function d(v) {
            const y = new Event("vite:preloadError", {
                cancelable: !0
            });
            if (y.payload = v, window.dispatchEvent(y), !y.defaultPrevented) throw v;
        }
        return c.then((v)=>{
            for (const y of v || [])y.status === "rejected" && d(y.reason);
            return i().catch(d);
        });
    };
    let rl = null, Ja = null;
    async function uS() {
        if (!rl) return Ja || (Ja = (async ()=>{
            try {
                const l = await iS(()=>import("./rlqt_ql_wasm-CbSHWufs.js"), []);
                await l.default(), rl = l;
            } catch (l) {
                console.warn("WASM module not available, validation disabled:", l), Ja = null;
            }
        })(), Ja);
    }
    function rg() {
        return rl !== null;
    }
    function sS(l) {
        return rl ? rl.validate_query(l) : {
            valid: !0,
            error_message: null,
            error_position: null,
            suggestions: []
        };
    }
    function rS() {
        return rl ? rl.get_autocomplete_data() : null;
    }
    const oS = [
        {
            query: ":errors",
            description: "All error logs"
        },
        {
            query: '@24h subsystem == "connections"',
            description: "Connection logs, last 24h"
        },
        {
            query: ":crashes | sort timestamp desc",
            description: "Crashes preset, newest first"
        },
        {
            query: "#tls",
            description: "TLS-related entries"
        },
        {
            query: "#connections and -#timeouts",
            description: "Connections without timeouts"
        },
        {
            query: 'message contains "timeout" | limit 50',
            description: 'Messages with "timeout"'
        },
        {
            query: '@1h severity == "warning" or severity == "error"',
            description: "Warnings/errors last hour"
        },
        {
            query: ":errors_or_crashes @7d",
            description: "Errors or crashes in the last week"
        }
    ];
    function cS({ query: l, limit: i, onQueryChange: u, onLimitChange: r, onRunQuery: c, isLoading: d }) {
        const [m, v] = ie.useState(null), [y, p] = ie.useState(!1), [_, M] = ie.useState(!1), [j, T] = ie.useState(!1), [U, q] = ie.useState(!1), [B, G] = ie.useState(null);
        ie.useEffect(()=>{
            uS().then(()=>{
                p(rg()), rg() && G(rS());
            });
        }, []), ie.useEffect(()=>{
            if (y && l.trim()) {
                const F = sS(l);
                v(F);
            } else l.trim() || v(null);
        }, [
            l,
            y
        ]);
        const ae = m?.valid ?? !0, Z = l.trim().length > 0 && (ae || !y) && !d, oe = ie.useCallback((F)=>{
            F.key === "Enter" && (F.preventDefault(), Z && c());
        }, [
            Z,
            c
        ]), he = (F)=>{
            u(F), M(!1);
        };
        return h.jsxs("div", {
            className: "bg-white shadow-sm border border-gray-200 rounded-lg p-4 space-y-4",
            children: [
                h.jsxs("div", {
                    className: "flex gap-3 items-start",
                    children: [
                        h.jsx("div", {
                            className: "flex-1",
                            children: h.jsxs("div", {
                                className: "relative",
                                children: [
                                    h.jsx("input", {
                                        type: "text",
                                        value: l,
                                        onChange: (F)=>u(F.target.value),
                                        onKeyDown: oe,
                                        placeholder: ":errors | limit 100",
                                        className: `w-full px-4 py-3 font-mono text-sm border rounded-lg focus:outline-none focus:ring-2 ${l.trim() && m && !m.valid ? "border-red-500 bg-red-50 focus:ring-red-500" : l.trim() && m?.valid ? "border-green-500 bg-green-50 focus:ring-green-500" : "border-gray-300 focus:ring-blue-500"}`
                                    }),
                                    l.trim() && m && h.jsx("div", {
                                        className: "absolute right-3 top-1/2 -translate-y-1/2",
                                        children: m.valid ? h.jsx("svg", {
                                            className: "w-5 h-5 text-green-500",
                                            fill: "currentColor",
                                            viewBox: "0 0 20 20",
                                            children: h.jsx("path", {
                                                fillRule: "evenodd",
                                                d: "M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z",
                                                clipRule: "evenodd"
                                            })
                                        }) : h.jsx("svg", {
                                            className: "w-5 h-5 text-red-500",
                                            fill: "currentColor",
                                            viewBox: "0 0 20 20",
                                            children: h.jsx("path", {
                                                fillRule: "evenodd",
                                                d: "M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z",
                                                clipRule: "evenodd"
                                            })
                                        })
                                    })
                                ]
                            })
                        }),
                        h.jsxs("select", {
                            value: i,
                            onChange: (F)=>r(parseInt(F.target.value)),
                            className: "px-3 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm",
                            title: "Result limit",
                            children: [
                                h.jsx("option", {
                                    value: 500,
                                    children: "500"
                                }),
                                h.jsx("option", {
                                    value: 1e3,
                                    children: "1000"
                                }),
                                h.jsx("option", {
                                    value: 3e3,
                                    children: "3000"
                                }),
                                h.jsx("option", {
                                    value: 5e3,
                                    children: "5000"
                                }),
                                h.jsx("option", {
                                    value: 1e4,
                                    children: "10000"
                                }),
                                h.jsx("option", {
                                    value: 2e4,
                                    children: "20000"
                                }),
                                h.jsx("option", {
                                    value: 5e4,
                                    children: "50000"
                                })
                            ]
                        }),
                        h.jsx("button", {
                            onClick: c,
                            disabled: !Z,
                            className: `px-6 py-3 rounded-lg font-medium text-white transition-colors ${Z ? "bg-blue-600 hover:bg-blue-700" : "bg-gray-400 cursor-not-allowed"}`,
                            children: d ? "Running..." : "Run"
                        })
                    ]
                }),
                h.jsxs("div", {
                    className: "flex items-center justify-between text-sm",
                    children: [
                        h.jsxs("div", {
                            className: "flex items-center gap-4",
                            children: [
                                h.jsx("button", {
                                    onClick: ()=>q(!U),
                                    className: "text-blue-600 hover:text-blue-800 font-medium",
                                    children: U ? "Hide Help" : "Help"
                                }),
                                h.jsx("button", {
                                    onClick: ()=>M(!_),
                                    className: "text-blue-600 hover:text-blue-800 font-medium",
                                    children: _ ? "Hide Examples" : "Examples"
                                }),
                                h.jsx("button", {
                                    onClick: ()=>T(!j),
                                    className: "text-blue-600 hover:text-blue-800 font-medium",
                                    children: j ? "Hide Reference" : "Quick Reference"
                                }),
                                B && h.jsxs("div", {
                                    className: "flex items-center gap-2 text-gray-500",
                                    children: [
                                        h.jsx("span", {
                                            children: "Presets:"
                                        }),
                                        B.presets.slice(0, 5).map((F)=>h.jsxs("button", {
                                                onClick: ()=>he(`:${F.name}`),
                                                className: "px-2 py-0.5 text-xs font-mono bg-gray-100 text-gray-700 rounded hover:bg-gray-200",
                                                title: F.description,
                                                children: [
                                                    ":",
                                                    F.name
                                                ]
                                            }, F.name))
                                    ]
                                })
                            ]
                        }),
                        h.jsx("div", {
                            className: "text-gray-500",
                            children: !y && h.jsxs("span", {
                                className: "text-yellow-600",
                                children: [
                                    h.jsx("svg", {
                                        className: "w-4 h-4 inline-block mr-1",
                                        fill: "currentColor",
                                        viewBox: "0 0 20 20",
                                        children: h.jsx("path", {
                                            fillRule: "evenodd",
                                            d: "M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z",
                                            clipRule: "evenodd"
                                        })
                                    }),
                                    "Server-side validation only"
                                ]
                            })
                        })
                    ]
                }),
                l.trim() && m && !m.valid && h.jsx("div", {
                    className: "bg-red-50 border border-red-200 rounded-lg p-3",
                    children: h.jsx("div", {
                        className: "flex items-start gap-2",
                        children: h.jsxs("div", {
                            className: "text-sm text-red-700",
                            children: [
                                h.jsx("span", {
                                    className: "font-medium",
                                    children: "Invalid query:"
                                }),
                                " ",
                                m.error_message,
                                m.suggestions.length > 0 && h.jsxs("span", {
                                    className: "ml-2",
                                    children: [
                                        "Did you mean: ",
                                        h.jsx("code", {
                                            className: "font-mono bg-red-100 px-1 rounded",
                                            children: m.suggestions[0]
                                        }),
                                        "?"
                                    ]
                                })
                            ]
                        })
                    })
                }),
                U && h.jsxs("div", {
                    className: "bg-blue-50 border border-blue-200 rounded-lg p-4 space-y-4",
                    children: [
                        h.jsxs("div", {
                            children: [
                                h.jsx("h3", {
                                    className: "text-sm font-semibold text-blue-800 mb-2",
                                    children: "Getting Started"
                                }),
                                h.jsxs("div", {
                                    className: "text-sm text-gray-700 space-y-2",
                                    children: [
                                        h.jsxs("p", {
                                            children: [
                                                "Type a query in the input field above, then press ",
                                                h.jsx("kbd", {
                                                    className: "px-1.5 py-0.5 bg-gray-200 rounded text-xs font-mono",
                                                    children: "Enter"
                                                }),
                                                " or click ",
                                                h.jsx("span", {
                                                    className: "font-medium",
                                                    children: "Run"
                                                }),
                                                " to search your logs."
                                            ]
                                        }),
                                        h.jsxs("p", {
                                            children: [
                                                h.jsx("span", {
                                                    className: "font-medium",
                                                    children: "Simplest query:"
                                                }),
                                                " Just type ",
                                                h.jsx("code", {
                                                    className: "bg-blue-100 px-1 rounded",
                                                    children: ":errors"
                                                }),
                                                " and press Enter to see all error logs. That's it!"
                                            ]
                                        })
                                    ]
                                })
                            ]
                        }),
                        h.jsxs("div", {
                            children: [
                                h.jsx("h3", {
                                    className: "text-sm font-semibold text-blue-800 mb-2",
                                    children: "Query Building Blocks"
                                }),
                                h.jsxs("div", {
                                    className: "text-sm text-gray-700 space-y-1.5",
                                    children: [
                                        h.jsxs("p", {
                                            children: [
                                                h.jsx("span", {
                                                    className: "font-medium text-blue-700",
                                                    children: "1. Presets"
                                                }),
                                                " start with ",
                                                h.jsx("code", {
                                                    className: "bg-blue-100 px-1 rounded",
                                                    children: ":"
                                                }),
                                                " — ready-made filters like ",
                                                h.jsx("code", {
                                                    className: "bg-blue-100 px-1 rounded",
                                                    children: ":errors"
                                                }),
                                                ",",
                                                " ",
                                                h.jsx("code", {
                                                    className: "bg-blue-100 px-1 rounded",
                                                    children: ":crashes"
                                                }),
                                                ", or",
                                                " ",
                                                h.jsx("code", {
                                                    className: "bg-blue-100 px-1 rounded",
                                                    children: ":errors_or_crashes"
                                                })
                                            ]
                                        }),
                                        h.jsxs("p", {
                                            children: [
                                                h.jsx("span", {
                                                    className: "font-medium text-blue-700",
                                                    children: "2. Time ranges"
                                                }),
                                                " start with ",
                                                h.jsx("code", {
                                                    className: "bg-blue-100 px-1 rounded",
                                                    children: "@"
                                                }),
                                                " — like ",
                                                h.jsx("code", {
                                                    className: "bg-blue-100 px-1 rounded",
                                                    children: "@1h"
                                                }),
                                                " (last hour),",
                                                " ",
                                                h.jsx("code", {
                                                    className: "bg-blue-100 px-1 rounded",
                                                    children: "@24h"
                                                }),
                                                " (last day), or",
                                                " ",
                                                h.jsx("code", {
                                                    className: "bg-blue-100 px-1 rounded",
                                                    children: "@7d"
                                                }),
                                                " (last week)"
                                            ]
                                        }),
                                        h.jsxs("p", {
                                            children: [
                                                h.jsx("span", {
                                                    className: "font-medium text-blue-700",
                                                    children: "3. Labels"
                                                }),
                                                " start with ",
                                                h.jsx("code", {
                                                    className: "bg-blue-100 px-1 rounded",
                                                    children: "#"
                                                }),
                                                " — like ",
                                                h.jsx("code", {
                                                    className: "bg-blue-100 px-1 rounded",
                                                    children: "#tls"
                                                }),
                                                ",",
                                                " ",
                                                h.jsx("code", {
                                                    className: "bg-blue-100 px-1 rounded",
                                                    children: "#connections"
                                                }),
                                                ", or exclude with",
                                                " ",
                                                h.jsx("code", {
                                                    className: "bg-blue-100 px-1 rounded",
                                                    children: "-#timeouts"
                                                })
                                            ]
                                        }),
                                        h.jsxs("p", {
                                            children: [
                                                h.jsx("span", {
                                                    className: "font-medium text-blue-700",
                                                    children: "4. Filters"
                                                }),
                                                " match specific fields — like ",
                                                h.jsx("code", {
                                                    className: "bg-blue-100 px-1 rounded",
                                                    children: 'severity == "error"'
                                                }),
                                                " or",
                                                " ",
                                                h.jsx("code", {
                                                    className: "bg-blue-100 px-1 rounded",
                                                    children: 'message contains "timeout"'
                                                })
                                            ]
                                        }),
                                        h.jsxs("p", {
                                            children: [
                                                h.jsx("span", {
                                                    className: "font-medium text-blue-700",
                                                    children: "5. Pipeline"
                                                }),
                                                " stages use ",
                                                h.jsx("code", {
                                                    className: "bg-blue-100 px-1 rounded",
                                                    children: "|"
                                                }),
                                                " — like ",
                                                h.jsx("code", {
                                                    className: "bg-blue-100 px-1 rounded",
                                                    children: "| limit 100"
                                                }),
                                                " or",
                                                " ",
                                                h.jsx("code", {
                                                    className: "bg-blue-100 px-1 rounded",
                                                    children: "| sort timestamp desc"
                                                })
                                            ]
                                        })
                                    ]
                                })
                            ]
                        }),
                        h.jsxs("div", {
                            children: [
                                h.jsx("h3", {
                                    className: "text-sm font-semibold text-blue-800 mb-2",
                                    children: "Combining Conditions"
                                }),
                                h.jsxs("div", {
                                    className: "text-sm text-gray-700 space-y-1.5",
                                    children: [
                                        h.jsxs("p", {
                                            children: [
                                                h.jsx("span", {
                                                    className: "font-medium text-blue-700",
                                                    children: "Chain building blocks:"
                                                }),
                                                " ",
                                                h.jsx("code", {
                                                    className: "bg-blue-100 px-1 rounded",
                                                    children: "@24h :crashes | limit 50"
                                                }),
                                                " — crashes from last 24h, limit to 50"
                                            ]
                                        }),
                                        h.jsxs("p", {
                                            children: [
                                                h.jsx("span", {
                                                    className: "font-medium text-blue-700",
                                                    children: "AND:"
                                                }),
                                                " ",
                                                h.jsx("code", {
                                                    className: "bg-blue-100 px-1 rounded",
                                                    children: 'severity == "error" and subsystem == "connections"'
                                                }),
                                                " — both must match"
                                            ]
                                        }),
                                        h.jsxs("p", {
                                            children: [
                                                h.jsx("span", {
                                                    className: "font-medium text-blue-700",
                                                    children: "OR:"
                                                }),
                                                " ",
                                                h.jsx("code", {
                                                    className: "bg-blue-100 px-1 rounded",
                                                    children: 'severity == "error" or severity == "warning"'
                                                }),
                                                " — either matches"
                                            ]
                                        }),
                                        h.jsxs("p", {
                                            children: [
                                                h.jsx("span", {
                                                    className: "font-medium text-blue-700",
                                                    children: "NOT:"
                                                }),
                                                " ",
                                                h.jsx("code", {
                                                    className: "bg-blue-100 px-1 rounded",
                                                    children: 'not severity == "debug"'
                                                }),
                                                " — excludes debug logs"
                                            ]
                                        }),
                                        h.jsxs("p", {
                                            children: [
                                                h.jsx("span", {
                                                    className: "font-medium text-blue-700",
                                                    children: "Grouping:"
                                                }),
                                                " ",
                                                h.jsx("code", {
                                                    className: "bg-blue-100 px-1 rounded",
                                                    children: '(severity == "error" or severity == "warning") and message contains "timeout"'
                                                })
                                            ]
                                        }),
                                        h.jsxs("p", {
                                            children: [
                                                h.jsx("span", {
                                                    className: "font-medium text-blue-700",
                                                    children: "Labels:"
                                                }),
                                                " ",
                                                h.jsx("code", {
                                                    className: "bg-blue-100 px-1 rounded",
                                                    children: "#tls or #connections"
                                                }),
                                                " — entries with either label,",
                                                " ",
                                                h.jsx("code", {
                                                    className: "bg-blue-100 px-1 rounded",
                                                    children: "#tls and -#timeouts"
                                                }),
                                                " — TLS without timeouts"
                                            ]
                                        })
                                    ]
                                })
                            ]
                        }),
                        h.jsxs("p", {
                            className: "text-xs text-gray-500 pt-2 border-t border-blue-200",
                            children: [
                                "Tip: Click ",
                                h.jsx("span", {
                                    className: "font-medium",
                                    children: "Examples"
                                }),
                                " to see more queries you can try, or click on any preset button above to use it directly."
                            ]
                        })
                    ]
                }),
                _ && h.jsxs("div", {
                    className: "bg-gray-50 border border-gray-200 rounded-lg p-4",
                    children: [
                        h.jsx("h3", {
                            className: "text-sm font-semibold text-gray-700 mb-3",
                            children: "Example Queries"
                        }),
                        h.jsx("div", {
                            className: "grid grid-cols-1 md:grid-cols-2 gap-2",
                            children: oS.map((F, $)=>h.jsxs("div", {
                                    className: "flex items-center justify-between gap-3 p-2 rounded hover:bg-gray-100 cursor-pointer",
                                    onClick: ()=>he(F.query),
                                    children: [
                                        h.jsx("code", {
                                            className: "text-sm font-mono text-blue-700 truncate",
                                            children: F.query
                                        }),
                                        h.jsx("span", {
                                            className: "text-xs text-gray-500 whitespace-nowrap",
                                            children: F.description
                                        })
                                    ]
                                }, $))
                        }),
                        B && h.jsxs("div", {
                            className: "mt-4 pt-4 border-t border-gray-200",
                            children: [
                                h.jsx("h4", {
                                    className: "text-sm font-semibold text-gray-700 mb-2",
                                    children: "All Presets"
                                }),
                                h.jsx("div", {
                                    className: "flex flex-wrap gap-2",
                                    children: B.presets.map((F)=>h.jsxs("button", {
                                            onClick: ()=>he(`:${F.name}`),
                                            className: "px-2 py-1 text-xs font-mono bg-blue-100 text-blue-700 rounded hover:bg-blue-200",
                                            title: F.description,
                                            children: [
                                                ":",
                                                F.name
                                            ]
                                        }, F.name))
                                })
                            ]
                        })
                    ]
                }),
                j && B && h.jsx("div", {
                    className: "bg-gray-50 border border-gray-200 rounded-lg p-4",
                    children: h.jsxs("div", {
                        className: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 text-sm text-gray-600",
                        children: [
                            h.jsxs("div", {
                                children: [
                                    h.jsx("span", {
                                        className: "font-medium text-gray-700",
                                        children: "Fields:"
                                    }),
                                    " ",
                                    B.fields.map((F)=>F.name).join(", ")
                                ]
                            }),
                            h.jsxs("div", {
                                children: [
                                    h.jsx("span", {
                                        className: "font-medium text-gray-700",
                                        children: "Severity:"
                                    }),
                                    " ",
                                    B.severities.join(", ")
                                ]
                            }),
                            h.jsxs("div", {
                                children: [
                                    h.jsx("span", {
                                        className: "font-medium text-gray-700",
                                        children: "Operators:"
                                    }),
                                    " ",
                                    B.operators.map((F)=>F.symbol).join(", ")
                                ]
                            }),
                            h.jsxs("div", {
                                children: [
                                    h.jsx("span", {
                                        className: "font-medium text-gray-700",
                                        children: "Pipeline:"
                                    }),
                                    " ",
                                    B.pipeline_stages.map((F)=>F.name).join(", ")
                                ]
                            }),
                            h.jsxs("div", {
                                children: [
                                    h.jsx("span", {
                                        className: "font-medium text-gray-700",
                                        children: "Duration:"
                                    }),
                                    " ",
                                    B.duration_units.map((F)=>F.suffix).join(", "),
                                    " ",
                                    "(e.g., @24h, @7d)"
                                ]
                            }),
                            h.jsxs("div", {
                                children: [
                                    h.jsx("span", {
                                        className: "font-medium text-gray-700",
                                        children: "Special:"
                                    }),
                                    " ",
                                    B.special_filters.join(", ")
                                ]
                            })
                        ]
                    })
                })
            ]
        });
    }
    const kl = "/api";
    async function fS(l) {
        const i = new URLSearchParams(Object.entries(l).filter(([c, d])=>d != null && d !== "").map(([c, d])=>[
                c,
                String(d)
            ])).toString(), u = `${kl}/logs${i ? `?${i}` : ""}`, r = await fetch(u);
        if (!r.ok) throw new Error(`Failed to query logs: ${r.statusText}`);
        return r.json();
    }
    async function dS() {
        const l = await fetch(`${kl}/metadata`);
        if (!l.ok) throw new Error(`Failed to fetch metadata: ${l.statusText}`);
        return l.json();
    }
    async function hS() {
        const l = await fetch(`${kl}/stats`);
        if (!l.ok) throw new Error(`Failed to fetch stats: ${l.statusText}`);
        return l.json();
    }
    async function gS() {
        const l = await fetch(`${kl}/file-metadata`);
        if (!l.ok) throw new Error(`Failed to fetch file metadata: ${l.statusText}`);
        return l.json();
    }
    async function mS(l, i = {}) {
        const u = new URLSearchParams(Object.entries(i).filter(([d, m])=>m != null && m !== "").map(([d, m])=>[
                d,
                String(m)
            ])).toString(), r = `${kl}/logs/preset/${l}${u ? `?${u}` : ""}`, c = await fetch(r);
        if (!c.ok) throw new Error(`Failed to query logs by preset: ${c.statusText}`);
        return c.json();
    }
    async function yS(l) {
        const i = new URLSearchParams(Object.entries(l).filter(([c, d])=>d != null && d !== "").map(([c, d])=>[
                c,
                String(d)
            ])).toString(), u = `${kl}/logs/ql?${i}`, r = await fetch(u);
        if (!r.ok) {
            const c = await r.json().catch(()=>({}));
            throw new Error(c.error || `Failed to query logs by QL: ${r.statusText}`);
        }
        return r.json();
    }
    function og() {
        const i = new URLSearchParams(window.location.search).get("tab");
        return i === "ql" ? "ql" : i === "metadata" ? "metadata" : i === "preset_errors_or_crashes" ? "preset_errors_or_crashes" : "filters";
    }
    function cg() {
        const l = new URLSearchParams(window.location.search);
        return {
            query: l.get("ql_query") || "",
            limit: parseInt(l.get("ql_limit") || "1000")
        };
    }
    function fg() {
        const l = new URLSearchParams(window.location.search), i = {
            limit: parseInt(l.get("limit") || "1000")
        };
        return l.has("since_time") && (i.since_time = l.get("since_time")), l.has("to_time") && (i.to_time = l.get("to_time")), l.has("severity") && (i.severity = l.get("severity")), l.has("erlang_pid") && (i.erlang_pid = l.get("erlang_pid")), l.has("node") && (i.node = l.get("node")), l.has("subsystem") && (i.subsystem = l.get("subsystem")), l.has("labels") && (i.labels = l.get("labels")), l.has("matching_all_labels") && (i.matching_all_labels = l.get("matching_all_labels") === "true"), l.has("has_resolution_or_discussion_url") && (i.has_resolution_or_discussion_url = l.get("has_resolution_or_discussion_url") === "true"), l.has("has_doc_url") && (i.has_doc_url = l.get("has_doc_url") === "true"), i;
    }
    function dg() {
        const l = new URLSearchParams(window.location.search), i = {
            limit: parseInt(l.get("preset_limit") || "1000")
        };
        return l.has("preset_since_time") && (i.since_time = l.get("preset_since_time")), l.has("preset_to_time") && (i.to_time = l.get("preset_to_time")), l.has("preset_node") && (i.node = l.get("preset_node")), i;
    }
    function vS(l, i, u, r) {
        const c = new URLSearchParams;
        return r !== "filters" && c.set("tab", r), r === "filters" ? (l.limit && c.set("limit", l.limit.toString()), l.since_time && c.set("since_time", l.since_time), l.to_time && c.set("to_time", l.to_time), l.severity && c.set("severity", l.severity), l.erlang_pid && c.set("erlang_pid", l.erlang_pid), l.node && c.set("node", l.node), l.subsystem && c.set("subsystem", l.subsystem), l.labels && c.set("labels", l.labels), l.matching_all_labels && c.set("matching_all_labels", "true"), l.has_resolution_or_discussion_url && c.set("has_resolution_or_discussion_url", "true"), l.has_doc_url && c.set("has_doc_url", "true")) : r === "ql" ? (u.query && c.set("ql_query", u.query), u.limit !== 1e3 && c.set("ql_limit", u.limit.toString())) : r === "preset_errors_or_crashes" && (i.limit && c.set("preset_limit", i.limit.toString()), i.since_time && c.set("preset_since_time", i.since_time), i.to_time && c.set("preset_to_time", i.to_time), i.node && c.set("preset_node", i.node)), c.toString();
    }
    function pS() {
        const [l, i] = ie.useState(()=>fg()), [u, r] = ie.useState(()=>dg()), [c, d] = ie.useState(()=>cg()), [m, v] = ie.useState(()=>og()), [y, p] = ie.useState(0);
        ie.useEffect(()=>{
            const I = ()=>{
                i(fg()), r(dg()), d(cg()), v(og());
            };
            return window.addEventListener("popstate", I), ()=>window.removeEventListener("popstate", I);
        }, []), ie.useEffect(()=>{
            const I = vS(l, u, c, m), Me = I ? `?${I}` : window.location.pathname;
            window.history.pushState({}, "", Me);
        }, [
            l,
            u,
            c,
            m
        ]);
        const _ = (I)=>{
            i((Me)=>({
                    ...Me,
                    erlang_pid: I
                }));
        }, { data: M } = Kl({
            queryKey: [
                "metadata"
            ],
            queryFn: dS
        }), { data: j } = Kl({
            queryKey: [
                "stats"
            ],
            queryFn: hS
        }), { data: T } = Kl({
            queryKey: [
                "fileMetadata"
            ],
            queryFn: gS
        }), { data: U, isLoading: q, error: B } = Kl({
            queryKey: [
                "logs",
                l
            ],
            queryFn: ()=>fS(l),
            enabled: m === "filters"
        }), { data: G, isLoading: ae, error: Z } = Kl({
            queryKey: [
                "preset_errors_or_crashes",
                u
            ],
            queryFn: ()=>mS("errors_or_crashes", u),
            enabled: m === "preset_errors_or_crashes"
        }), { data: oe, isLoading: he, error: F } = Kl({
            queryKey: [
                "ql",
                c.query,
                c.limit,
                y
            ],
            queryFn: ()=>yS({
                    query: c.query,
                    limit: c.limit
                }),
            enabled: m === "ql" && c.query.trim().length > 0 && y > 0
        }), $ = ie.useCallback(()=>{
            p((I)=>I + 1);
        }, []);
        return h.jsxs("div", {
            className: "min-h-screen bg-gray-50",
            children: [
                h.jsx("header", {
                    className: "bg-white shadow-sm border-b border-gray-200",
                    children: h.jsx("div", {
                        className: "mx-auto px-4 sm:px-6 lg:px-8 py-4",
                        children: h.jsx("div", {
                            className: "flex items-center justify-between",
                            children: h.jsxs("div", {
                                children: [
                                    h.jsx("h1", {
                                        className: "text-2xl font-bold text-gray-900",
                                        children: "RabbitMQ Log Query Tools"
                                    }),
                                    h.jsx("p", {
                                        className: "text-sm text-gray-500 mt-1",
                                        children: "RabbitMQ logs. You dig?"
                                    })
                                ]
                            })
                        })
                    })
                }),
                h.jsxs("main", {
                    className: "mx-auto px-4 sm:px-6 lg:px-8 py-8",
                    children: [
                        h.jsx("div", {
                            className: "border-b border-gray-200 mb-6",
                            children: h.jsxs("nav", {
                                className: "-mb-px flex space-x-8",
                                children: [
                                    h.jsx("button", {
                                        onClick: ()=>v("filters"),
                                        className: `${m === "filters" ? "border-blue-500 text-blue-600" : "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300"} whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm transition-colors`,
                                        children: "Filters"
                                    }),
                                    h.jsx("button", {
                                        onClick: ()=>v("ql"),
                                        className: `${m === "ql" ? "border-blue-500 text-blue-600" : "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300"} whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm transition-colors`,
                                        children: "QL"
                                    }),
                                    h.jsx("button", {
                                        onClick: ()=>v("metadata"),
                                        className: `${m === "metadata" ? "border-blue-500 text-blue-600" : "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300"} whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm transition-colors`,
                                        children: "File Set Overview"
                                    }),
                                    h.jsx("button", {
                                        onClick: ()=>v("preset_errors_or_crashes"),
                                        className: `${m === "preset_errors_or_crashes" ? "border-blue-500 text-blue-600" : "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300"} whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm transition-colors`,
                                        children: "Preset 1: Errors, Exceptions"
                                    })
                                ]
                            })
                        }),
                        T && T.length > 0 && j && h.jsx("div", {
                            className: "bg-white shadow-sm border border-gray-200 rounded-lg p-4 mb-6",
                            children: h.jsxs("div", {
                                className: "flex items-center justify-between",
                                children: [
                                    h.jsxs("div", {
                                        children: [
                                            h.jsx("div", {
                                                className: "text-sm text-gray-500",
                                                children: "Time Range"
                                            }),
                                            h.jsx("div", {
                                                className: "text-sm text-gray-700 mt-1",
                                                children: T[0].oldest_entry_at && T[0].most_recent_entry_at && h.jsxs(h.Fragment, {
                                                    children: [
                                                        Jl(T[0].oldest_entry_at),
                                                        " — ",
                                                        Jl(T[0].most_recent_entry_at)
                                                    ]
                                                })
                                            })
                                        ]
                                    }),
                                    h.jsxs("div", {
                                        className: "text-right",
                                        children: [
                                            h.jsx("div", {
                                                className: "text-sm text-gray-500",
                                                children: "Total Entries"
                                            }),
                                            h.jsx("div", {
                                                className: "text-2xl font-bold text-gray-900 mt-1",
                                                children: j.total_entries.toLocaleString()
                                            })
                                        ]
                                    })
                                ]
                            })
                        }),
                        m === "filters" && h.jsxs("div", {
                            className: "grid grid-cols-1 lg:grid-cols-6 gap-6",
                            children: [
                                h.jsx("div", {
                                    className: "lg:col-span-1",
                                    children: h.jsx(Py, {
                                        metadata: M || null,
                                        filters: l,
                                        onFilterChange: i
                                    })
                                }),
                                h.jsxs("div", {
                                    className: "lg:col-span-5 space-y-4",
                                    children: [
                                        h.jsx(tS, {
                                            fileMetadata: T,
                                            onViewFullMetadata: ()=>v("metadata")
                                        }),
                                        h.jsx("div", {
                                            className: "bg-white shadow-sm border border-gray-200 rounded-lg p-4",
                                            children: h.jsx("div", {
                                                className: "flex items-center justify-between",
                                                children: h.jsx("div", {
                                                    className: "text-sm text-gray-700",
                                                    children: q ? h.jsxs("span", {
                                                        className: "flex items-center gap-2",
                                                        children: [
                                                            h.jsxs("svg", {
                                                                className: "animate-spin h-4 w-4 text-blue-600",
                                                                xmlns: "http://www.w3.org/2000/svg",
                                                                fill: "none",
                                                                viewBox: "0 0 24 24",
                                                                children: [
                                                                    h.jsx("circle", {
                                                                        className: "opacity-25",
                                                                        cx: "12",
                                                                        cy: "12",
                                                                        r: "10",
                                                                        stroke: "currentColor",
                                                                        strokeWidth: "4"
                                                                    }),
                                                                    h.jsx("path", {
                                                                        className: "opacity-75",
                                                                        fill: "currentColor",
                                                                        d: "M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                                                                    })
                                                                ]
                                                            }),
                                                            "Loading..."
                                                        ]
                                                    }) : B ? h.jsxs("span", {
                                                        className: "text-red-600",
                                                        children: [
                                                            "Error: ",
                                                            B.message
                                                        ]
                                                    }) : h.jsxs("span", {
                                                        children: [
                                                            "Showing ",
                                                            h.jsx("span", {
                                                                className: "font-semibold",
                                                                children: U?.total || 0
                                                            }),
                                                            " matching",
                                                            " ",
                                                            U?.total === 1 ? "entry" : "entries"
                                                        ]
                                                    })
                                                })
                                            })
                                        }),
                                        h.jsx(io, {
                                            data: U?.entries || [],
                                            onPidFilterClick: _
                                        })
                                    ]
                                })
                            ]
                        }),
                        m === "ql" && h.jsxs("div", {
                            className: "space-y-4",
                            children: [
                                h.jsx(cS, {
                                    query: c.query,
                                    limit: c.limit,
                                    onQueryChange: (I)=>d((Me)=>({
                                                ...Me,
                                                query: I
                                            })),
                                    onLimitChange: (I)=>d((Me)=>({
                                                ...Me,
                                                limit: I
                                            })),
                                    onRunQuery: $,
                                    isLoading: he
                                }),
                                h.jsx("div", {
                                    className: "bg-white shadow-sm border border-gray-200 rounded-lg p-4",
                                    children: h.jsx("div", {
                                        className: "flex items-center justify-between",
                                        children: h.jsx("div", {
                                            className: "text-sm text-gray-700",
                                            children: he ? h.jsxs("span", {
                                                className: "flex items-center gap-2",
                                                children: [
                                                    h.jsxs("svg", {
                                                        className: "animate-spin h-4 w-4 text-blue-600",
                                                        xmlns: "http://www.w3.org/2000/svg",
                                                        fill: "none",
                                                        viewBox: "0 0 24 24",
                                                        children: [
                                                            h.jsx("circle", {
                                                                className: "opacity-25",
                                                                cx: "12",
                                                                cy: "12",
                                                                r: "10",
                                                                stroke: "currentColor",
                                                                strokeWidth: "4"
                                                            }),
                                                            h.jsx("path", {
                                                                className: "opacity-75",
                                                                fill: "currentColor",
                                                                d: "M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                                                            })
                                                        ]
                                                    }),
                                                    "Running query..."
                                                ]
                                            }) : F ? h.jsxs("span", {
                                                className: "text-red-600",
                                                children: [
                                                    "Error: ",
                                                    F.message
                                                ]
                                            }) : oe ? h.jsxs("span", {
                                                children: [
                                                    "Showing ",
                                                    h.jsx("span", {
                                                        className: "font-semibold",
                                                        children: oe.total
                                                    }),
                                                    " matching",
                                                    " ",
                                                    oe.total === 1 ? "entry" : "entries"
                                                ]
                                            }) : h.jsx("span", {
                                                className: "text-gray-500",
                                                children: "Enter a query and click Run"
                                            })
                                        })
                                    })
                                }),
                                h.jsx(io, {
                                    data: oe?.entries || [],
                                    onPidFilterClick: _
                                })
                            ]
                        }),
                        m === "preset_errors_or_crashes" && h.jsxs("div", {
                            className: "grid grid-cols-1 lg:grid-cols-6 gap-6",
                            children: [
                                h.jsx("div", {
                                    className: "lg:col-span-1",
                                    children: h.jsx(nS, {
                                        metadata: M || null,
                                        filters: u,
                                        onFilterChange: r
                                    })
                                }),
                                h.jsxs("div", {
                                    className: "lg:col-span-5 space-y-4",
                                    children: [
                                        h.jsx("div", {
                                            className: "bg-white shadow-sm border border-gray-200 rounded-lg p-4",
                                            children: h.jsx("div", {
                                                className: "flex items-center justify-between",
                                                children: h.jsx("div", {
                                                    className: "text-sm text-gray-700",
                                                    children: ae ? h.jsxs("span", {
                                                        className: "flex items-center gap-2",
                                                        children: [
                                                            h.jsxs("svg", {
                                                                className: "animate-spin h-4 w-4 text-blue-600",
                                                                xmlns: "http://www.w3.org/2000/svg",
                                                                fill: "none",
                                                                viewBox: "0 0 24 24",
                                                                children: [
                                                                    h.jsx("circle", {
                                                                        className: "opacity-25",
                                                                        cx: "12",
                                                                        cy: "12",
                                                                        r: "10",
                                                                        stroke: "currentColor",
                                                                        strokeWidth: "4"
                                                                    }),
                                                                    h.jsx("path", {
                                                                        className: "opacity-75",
                                                                        fill: "currentColor",
                                                                        d: "M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                                                                    })
                                                                ]
                                                            }),
                                                            "Loading..."
                                                        ]
                                                    }) : Z ? h.jsxs("span", {
                                                        className: "text-red-600",
                                                        children: [
                                                            "Error: ",
                                                            Z.message
                                                        ]
                                                    }) : h.jsxs("span", {
                                                        children: [
                                                            "Showing ",
                                                            h.jsx("span", {
                                                                className: "font-semibold",
                                                                children: G?.total || 0
                                                            }),
                                                            " matching",
                                                            " ",
                                                            G?.total === 1 ? "entry" : "entries"
                                                        ]
                                                    })
                                                })
                                            })
                                        }),
                                        h.jsx(io, {
                                            data: G?.entries || [],
                                            onPidFilterClick: _
                                        })
                                    ]
                                })
                            ]
                        }),
                        m === "metadata" && h.jsx(eS, {
                            fileMetadata: T
                        })
                    ]
                })
            ]
        });
    }
    const SS = new Uy({
        defaultOptions: {
            queries: {
                refetchOnWindowFocus: !1,
                retry: 1
            }
        }
    });
    hy.createRoot(document.getElementById("root")).render(h.jsx(ie.StrictMode, {
        children: h.jsx(Wy, {
            children: h.jsx(Ly, {
                client: SS,
                children: h.jsx(pS, {})
            })
        })
    }));
})();
