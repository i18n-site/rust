import BODY from "@2-/doc/BODY.js";

import goto from "./goto.js";

import selfA from "./selfA.js";

BODY.addEventListener("click", (e) => {
	var href, name, p, results;
	p = e.target;
	results = [];
	while (p) {
		({ nodeName: name } = p);
		if (name === "A") {
			({ href } = p);
			if (href) {
				href = selfA(p, e);
				if (href !== void 0) {
					goto(href);
				} else if (!p.target) {
					p.target = "_blank";
				}
			}
			break;
		} else if (name === "BODY") {
			break;
		}
		results.push((p = p.parentNode));
	}
	return results;
});

const abc = 1;

export default (ef) => {
	let xxx = 1;
	return abc + ef + xxx;
};
