<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>create health</name>
   <tag></tag>
   <elementGuidId>02e4af46-1cd2-4fee-b1dc-6a0af1021f9f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;email\&quot; : \&quot;rs_CeriaMedika@gmail.com\&quot;,\n    \&quot;username\&quot;:\&quot;rsCeriaMedika\&quot;,\n    \&quot;password\&quot; : \&quot;ceriaSelalu123\&quot;,    \n    \&quot;role\&quot; : \&quot;admin\&quot;,\n    \&quot;noHp\&quot;: \&quot;02292\&quot;,\n    \&quot;nama\&quot;: \&quot;RS Ceria\&quot;,\n    \&quot;image\&quot;: \&quot;rsCeria.jpg\&quot;,\n    \&quot;address\&quot;: \&quot;JL Flamboyan\&quot;,\n    \&quot;kota\&quot;: \&quot;Bali\&quot;\n}&quot;,
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
      <webElementGuid>97d8a559-afaa-4ebb-ac99-ce6748a4d1dc</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://booking-vaksin-alta.herokuapp.com/api/user</restUrl>
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
