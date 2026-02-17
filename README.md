
# 🎭 Gambas Matcher Mock

**Gambas Matcher Mock** is a lightweight and intuitive library designed to simplify unit testing and mocking in Gambas 3. It allows you to precisely verify arguments passed to your mocked objects.

---

## ✨ Key Features

* ✅ **Fluent Syntax:** A highly readable API to define your expectations.
* 🚀 **Lightweight:** No complex external dependencies.
* 🛠️ **Extensible:** Easily create your own custom matchers.
* 🧪 **Quality Focused:** Designed to integrate seamlessly with your existing test suites.

---

## 🚀 Installation

To integrate Matcher Mock into your Gambas project:

1. Download the `.class` files from the `src/` directory.
2. Import them into your Gambas project using the IDE.
3. *Optional:* If you use a package manager (e.g., `gb-pkg`), add the corresponding dependency.

---

## 💡 Usage Example

Here is how to use the matcher to verify a method call:

```gambas
' Example of argument verification
Public Sub Test_MyMethod_With_Matcher()
  Dim hMock As New MyMockObject
  
  ' Call the method to be tested
  hMock.ProcessData("Hello World", 42)
  
  ' Verification using the Matcher
  Assert.IsTrue(Matcher.Expect("Hello*").Matches(hMock.LastArg(0)))
  Assert.IsTrue(Matcher.Between(40, 50).Matches(hMock.LastArg(1)))
End Sub

```

---

## 🛠 Project Structure

* `src/` : Contains the source classes for the Matcher and Mock.
* `tests/` : Unit test examples validating the library's behavior.
* `examples/` : Sample projects for a quick start.

---

## 🤝 Contributing

Contributions are welcome!

1. Check the [CONTRIBUTING.md]() file.
2. Open an **Issue** to discuss a proposed change.
3. Submit a **Pull Request**.

---

## 📄 License

This project is licensed under the **MIT License**. See the [LICENSE]() file for details.

---

**Author:** [@lionelkouame]()

---

### Pourquoi ces changements ?

* **"Key Features"** au lieu de "Points forts" : C'est le terme standard utilisé dans la documentation technique.
* **"Usage Example"** : Plus naturel que "Example of use".
* **Terminologie technique** : J'ai conservé les termes comme *fluent syntax* et *mocking* car ce sont des concepts universels pour les développeurs.