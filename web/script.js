document.getElementById("runButton").addEventListener("click", async function () {
  const sourceCode = document.getElementById("sourceCode").value;

  try {
    // ここでプログラムを実行する関数を呼び出します。
    // 例: const result = await executeProgram(sourceCode);
    // 現在は、正常終了と異常終了のデモンストレーションを行います。
    const isError = Math.random() < 0.5;
    const result = isError ? "エラーが発生しました。" : "正常に実行されました。";

    const outputItem = document.createElement("div");
    outputItem.classList.add("outputItem");
    if (isError) {
      outputItem.classList.add("error");
    }

    outputItem.textContent = `入力コード:\n${sourceCode}\n実行結果:\n${result}\n`;
    document.getElementById("outputContainer").appendChild(outputItem);

    // 入力欄をクリア
    document.getElementById("sourceCode").value = "";
  } catch (error) {
    console.error("プログラムの実行中にエラーが発生しました。", error);
  }
});

