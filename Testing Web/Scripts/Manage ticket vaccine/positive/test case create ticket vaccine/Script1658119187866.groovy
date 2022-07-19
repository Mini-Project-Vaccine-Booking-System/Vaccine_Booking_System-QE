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

WebUI.openBrowser('https://test-vaksinqu.vercel.app/fitur/tiketVaksin')

WebUI.maximizeWindow()

WebUI.setText(findTestObject('Login/positive/input email'), 'rsdharmawangsa@gmail.com')

WebUI.setText(findTestObject('Login/positive/input password'), 'semarang')

WebUI.click(findTestObject('Login/positive/button login'))

WebUI.click(findTestObject('ticket vaccine/positive/button ticket vaccine'))

WebUI.setText(findTestObject('ticket vaccine/positive/input date'), '17/07/2022')

WebUI.setText(findTestObject('ticket vaccine/positive/input waku awal'), '09:00')

WebUI.setText(findTestObject('ticket vaccine/positive/input waktu akhir'), '10:00')

WebUI.click(findTestObject('ticket vaccine/positive/button selanjutnya'))

WebUI.click(findTestObject('ticket vaccine/positive/input jenis vaksin'), FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('ticket vaccine/positive/vaccine sinovac'))

WebUI.setText(findTestObject('ticket vaccine/positive/input stok vaksin'), '100')

WebUI.click(findTestObject('ticket vaccine/positive/button selanjutnya'))

WebUI.click(findTestObject('ticket vaccine/positive/button kirim'))

WebUI.verifyElementText(findTestObject('ticket vaccine/positive/verify success ticket vaccine'), 'Tiket Vaksin Berhasil Di Terbitkan')

WebUI.delay(5)

WebUI.closeBrowser()

