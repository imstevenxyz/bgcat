"use strict";

/*
 * Register form expansions
 */
(function () {
  modifyBoardgameForm();
  modifyBoardgameExpansionButtons();
  console.log("bgcat: Registered add expansion button");
})();

/*
 * Replace `exp-add` checkbox with a button
 */
function modifyBoardgameForm() {
  const button = document.createElement("button");
  button.innerText = "Add expansion";
  button.id = "exp-add";
  button.type = "button";
  button.addEventListener("click", () => {
    addBoardgameExpansion();
  });
  document.getElementById("exp-control").replaceWith(button);
}

/*
 * Replace `exp-del` checkbox with a button
 */
function modifyBoardgameExpansionButtons() {
  document.querySelectorAll("#exp-del-control").forEach((form) => {
    const button = document.createElement("button");
    button.innerHTML = '<i class="fas fa-solid fa-trash"></i> delete';
    button.id = "exp-del";
    button.type = "button";
    button.addEventListener("click", () => {
      if (confirm("Delete expansion?")) {
        button.parentElement.parentElement.remove();
      }
    });
    form.replaceChildren(button);
  });
}

/*
 * Add new expanion to the form
 */
function addBoardgameExpansion() {
  container = document.getElementById("exp-container");
  container.insertAdjacentHTML(
    "beforeend",
    `
    <div class="exp">
      <h4>Expansion:</h4>
      <div>
          <label for="exp-title">Title:</label>
          <input id="exp-title" name="expansions[][title]" type="text" required>
      </div>
      <div id="exp-del-control"></div>
    </div>
  `,
  );
  modifyBoardgameExpansionButtons();
}
