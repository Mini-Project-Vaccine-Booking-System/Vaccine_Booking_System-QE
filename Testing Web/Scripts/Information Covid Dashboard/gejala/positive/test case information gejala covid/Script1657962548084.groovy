import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('https://test-vaksinqu.vercel.app/')

WebUI.maximizeWindow()

WebUI.setText(findTestObject('Login/positive/input email'), 'rsdharmawangsa@gmail.com')

WebUI.setText(findTestObject('Login/positive/input password'), 'semarang')

WebUI.click(findTestObject('Login/positive/button login'))

WebUI.scrollToElement(findTestObject('Information covid/gejala/positive/verify gejala'), 3)

WebUI.verifyElementText(findTestObject('Information covid/gejala/positive/verify gejala'), 'Segera cari bantuan medis jika Anda mengalami gejala serius. Selalu hubungi dokter atau fasilitas kesehatan sebelum mengunjunginya. Orang dengan gejala ringan yang dinyatakan sehat harus melakukan perawatan mandiri di rumah. Rata-rata gejala akan muncul 5–6 hari setelah seseorang pertama kali terinfeksi virus ini, tetapi bisa juga sampai 14 hari setelah terinfeksi.')

WebUI.delay(5)

WebUI.closeBrowser()

