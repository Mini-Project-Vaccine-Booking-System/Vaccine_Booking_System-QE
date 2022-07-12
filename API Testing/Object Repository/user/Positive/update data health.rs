<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>update data health</name>
   <tag></tag>
   <elementGuidId>e0063db8-cf51-4d47-87e4-da773ee3b63b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;email\&quot; : \&quot;RS_Flamboyan@gmail.com\&quot;,\n    \&quot;username\&quot;:\&quot;rsFlamboyan\&quot;,\n    \&quot;password\&quot; : \&quot;rsFlamboyan123\&quot;,    \n    \&quot;role\&quot; : \&quot;admin\&quot;,\n    \&quot;noHp\&quot;: \&quot;0222123456\&quot;,\n    \&quot;nama\&quot;: \&quot;RS Flamboyan\&quot;,\n    \&quot;image\&quot;: \&quot;rsFlamboyan.jpg\&quot;,\n    \&quot;address\&quot;: \&quot;JL Flamboyan\&quot;,\n    \&quot;kota\&quot;: \&quot;Yogyakarta\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>cf1249d4-f031-424c-983f-02be5220675c</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>https://booking-vaksin-alta.herokuapp.com/api/user/14</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
