
if (process.env.NODE_ENV === "production") {
    console.log(" [*] Waiting 5 seconds for private networking");
    await new Promise((r) => setTimeout(r, 5000));
}
