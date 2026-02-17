# 🎭 Gambas Matcher Mock

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Gambas Version](https://img.shields.io/badge/Gambas-3.19+-blue.svg)](http://gambas.sourceforge.net/)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](http://makeapullrequest.com)

**Gambas Matcher Mock** est une bibliothèque légère et intuitive pour simplifier les tests unitaires et le mocking en Gambas 3. Elle permet de vérifier avec précision les arguments passés à vos objets mockés.

---

## ✨ Points forts

* ✅ **Syntaxe Fluide :** Une API lisible pour définir vos attentes.
* 🚀 **Léger :** Aucune dépendance externe complexe.
* 🛠️ **Extensible :** Créez vos propres matchers personnalisés facilement.
* 🧪 **Focus Qualité :** Conçu pour s'intégrer parfaitement avec vos suites de tests existantes.

---

## 🚀 Installation

Pour intégrer le Matcher Mock à votre projet Gambas :

1. Téléchargez les fichiers `.class` du dossier `src/`.
2. Importez-les dans votre projet Gambas via l'IDE.
3. *Optionnel :* Si vous utilisez un gestionnaire de paquets (ex: gb-pkg), ajoutez la dépendance correspondante.

---

## 💡 Exemple d'utilisation

Voici comment utiliser le matcher pour vérifier un appel de méthode :

```gambas
' Exemple de vérification d'argument
Public Sub Test_MyMethod_With_Matcher()
  Dim hMock As New MyMockObject
  
  ' Appel de la méthode à tester
  hMock.ProcessData("Hello World", 42)
  
  ' Vérification avec le Matcher
  Assert.IsTrue(Matcher.Expect("Hello*").Matches(hMock.LastArg(0)))
  Assert.IsTrue(Matcher.Between(40, 50).Matches(hMock.LastArg(1)))
End Sub