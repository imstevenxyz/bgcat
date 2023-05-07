/*
 * Register form expansions
 */
(function () {
  modify_bg_form();
  modify_exps();
  console.log("bgcat: Registered add expansion button");
})();

/*
 * Replace `exp-add` checkbox with a button
 */
function modify_bg_form() {
  const button = document.createElement("button");
  button.innerText = "Add expansion";
  button.id = "exp-add";
  button.type = "button";
  button.addEventListener("click", () => {
    add_expansion();
  });
  document.getElementById("exp-control").replaceWith(button);
}

/*
 * Replace `exp-del` checkbox with a button
 */
function modify_exps() {
  document.querySelectorAll("#exp-del-control").forEach((form) => {
    const button = document.createElement("button");
    button.innerHTML = '<i class="fas fa-solid fa-trash"></i> delete';
    button.id = "exp-del";
    button.type = "button";
    button.addEventListener("click", () => {
        if(confirm(`Delete expansion?`)){
            button.parentElement.parentElement.remove();
        }
    });
    form.replaceChildren(button);
  })
}

/*
 * Add new expanion to the form
 */
function add_expansion() {
  container = document.getElementById("exp-container");
  container.insertAdjacentHTML("beforeend",`
    <div class="exp">
      <h4>Expansion:</h4>
      <div>
          <label for="exp-title">Title:</label>
          <input id="exp-title" name="expansions[][title]" type="text" required>
      </div>
      <div>
        <label>Players:</label>
        <input id="exp-min-players" name="expansions[][min-player]" type="number" min="0" placeholder="min" aria-label="minimum players" required>
        &mdash;
        <input id="exp-max-players" name="expansions[][max-player]" type="number" min="0" placeholder="max" aria-label="maximum players" required>
      </div>
      <div id="exp-del-control"></div>
    </div>
  `);
  modify_exps();
}