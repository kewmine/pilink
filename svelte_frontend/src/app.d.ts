import pilink from "./pilink.svelte";

const app = new pilink({
	target: document.getElementById("pilink")
});

export default pilink;